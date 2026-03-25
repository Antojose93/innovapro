use sqlx::{Row, SqlitePool};

use crate::errors::AppResult;

const SCHEMA_STATEMENTS: [&str; 13] = [
    r#"
    CREATE TABLE IF NOT EXISTS units (
      id INTEGER PRIMARY KEY AUTOINCREMENT,
      code TEXT NOT NULL UNIQUE,
      name TEXT NOT NULL,
      symbol TEXT NOT NULL,
      created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
    )
    "#,
    r#"
    CREATE TABLE IF NOT EXISTS materials (
      id INTEGER PRIMARY KEY AUTOINCREMENT,
      code TEXT NOT NULL UNIQUE,
      name TEXT NOT NULL,
      stock_unit_id INTEGER NOT NULL,
      purchase_unit_id INTEGER NOT NULL,
      created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
      FOREIGN KEY (stock_unit_id) REFERENCES units(id),
      FOREIGN KEY (purchase_unit_id) REFERENCES units(id)
    )
    "#,
    r#"
    CREATE TABLE IF NOT EXISTS material_cost_history (
      id INTEGER PRIMARY KEY AUTOINCREMENT,
      material_id INTEGER NOT NULL,
      unit_id INTEGER NOT NULL,
      unit_cost REAL NOT NULL CHECK (unit_cost >= 0),
      effective_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
      FOREIGN KEY (material_id) REFERENCES materials(id) ON DELETE CASCADE,
      FOREIGN KEY (unit_id) REFERENCES units(id)
    )
    "#,
    r#"
    CREATE TABLE IF NOT EXISTS finished_products (
      id INTEGER PRIMARY KEY AUTOINCREMENT,
      code TEXT NOT NULL UNIQUE,
      name TEXT NOT NULL,
      unit_id INTEGER NOT NULL,
      created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
      FOREIGN KEY (unit_id) REFERENCES units(id)
    )
    "#,
    r#"
    CREATE TABLE IF NOT EXISTS unit_conversions (
      id INTEGER PRIMARY KEY AUTOINCREMENT,
      from_unit_id INTEGER NOT NULL,
      to_unit_id INTEGER NOT NULL,
      factor REAL NOT NULL CHECK (factor > 0),
      created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
      FOREIGN KEY (from_unit_id) REFERENCES units(id),
      FOREIGN KEY (to_unit_id) REFERENCES units(id),
      UNIQUE (from_unit_id, to_unit_id)
    )
    "#,
    r#"
    CREATE TABLE IF NOT EXISTS product_bom_items (
      id INTEGER PRIMARY KEY AUTOINCREMENT,
      product_id INTEGER NOT NULL,
      material_id INTEGER NOT NULL,
      quantity REAL NOT NULL CHECK (quantity > 0),
      created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
      FOREIGN KEY (product_id) REFERENCES finished_products(id) ON DELETE CASCADE,
      FOREIGN KEY (material_id) REFERENCES materials(id),
      UNIQUE (product_id, material_id)
    )
    "#,
    r#"
    CREATE TABLE IF NOT EXISTS recipes (
      id INTEGER PRIMARY KEY AUTOINCREMENT,
      code TEXT NOT NULL UNIQUE,
      name TEXT NOT NULL,
      product_id INTEGER,
      status TEXT NOT NULL DEFAULT 'active',
      version TEXT NOT NULL DEFAULT 'v1',
      output_quantity REAL NOT NULL DEFAULT 1,
      output_unit_id INTEGER NOT NULL DEFAULT 5,
      notes TEXT,
      created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
      FOREIGN KEY (product_id) REFERENCES finished_products(id) ON DELETE SET NULL,
      FOREIGN KEY (output_unit_id) REFERENCES units(id)
    )
    "#,
    r#"
    CREATE TABLE IF NOT EXISTS recipe_items (
      id INTEGER PRIMARY KEY AUTOINCREMENT,
      recipe_id INTEGER NOT NULL,
      material_id INTEGER NOT NULL,
      quantity REAL NOT NULL CHECK (quantity > 0),
      unit_id INTEGER NOT NULL DEFAULT 1,
      created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
      FOREIGN KEY (recipe_id) REFERENCES recipes(id) ON DELETE CASCADE,
      FOREIGN KEY (material_id) REFERENCES materials(id),
      FOREIGN KEY (unit_id) REFERENCES units(id),
      UNIQUE (recipe_id, material_id)
    )
    "#,
    "CREATE INDEX IF NOT EXISTS idx_material_cost_history_material_id ON material_cost_history(material_id)",
    "CREATE INDEX IF NOT EXISTS idx_material_cost_history_effective_at ON material_cost_history(effective_at DESC)",
    "CREATE INDEX IF NOT EXISTS idx_unit_conversions_pair ON unit_conversions(from_unit_id, to_unit_id)",
    "CREATE INDEX IF NOT EXISTS idx_product_bom_items_product_id ON product_bom_items(product_id)",
    "CREATE INDEX IF NOT EXISTS idx_recipe_items_recipe_id ON recipe_items(recipe_id)",
];

const DEFAULT_UNITS: [(&str, &str, &str); 5] = [
    ("KG", "Kilogramo", "kg"),
    ("G", "Gramo", "g"),
    ("L", "Litro", "L"),
    ("ML", "Mililitro", "ml"),
    ("UND", "Unidad", "und"),
];

const SEEDED_UNITS: [(&str, &str, &str); 9] = [
    ("GAL", "Galon", "gal"),
    ("M", "Metro", "m"),
    ("CM", "Centimetro", "cm"),
    ("MM", "Milimetro", "mm"),
    ("VAR6", "Varilla (6 metros)", "var-6m"),
    ("PLA6", "Platina (6 metros)", "plat-6m"),
    ("TUB6", "Tubo (6 metros)", "tubo-6m"),
    ("CAJ5", "Caja (5 kg)", "caja-5kg"),
    ("UND", "Unidad", "und"),
];

const SEEDED_MATERIALS: [(&str, &str, &str, &str, f64, &str); 15] = [
    (
        "HIE-001",
        "Varilla Corrugada 3/8\" (9.5mm)",
        "UND",
        "VAR6",
        18500.0,
        "VAR6",
    ),
    (
        "HIE-002",
        "Varilla Corrugada 1/2\" (12.7mm)",
        "UND",
        "VAR6",
        32500.0,
        "VAR6",
    ),
    (
        "HIE-003",
        "Varilla Cuadrada Lisa 1/2\" (12mm)",
        "UND",
        "VAR6",
        38500.0,
        "VAR6",
    ),
    (
        "HIE-004",
        "Platina de Hierro 1\" x 1/8\"",
        "UND",
        "PLA6",
        24000.0,
        "PLA6",
    ),
    (
        "HIE-005",
        "Platina de Hierro 2\" x 1/8\"",
        "UND",
        "PLA6",
        48500.0,
        "PLA6",
    ),
    (
        "HIE-006",
        "Tubo Cuadrado (Perfil) 1.5\" x 1.5\" Calibre 18",
        "UND",
        "TUB6",
        51500.0,
        "TUB6",
    ),
    (
        "SOL-001",
        "Electrodo E6013 de 1/8\" (Caja)",
        "KG",
        "CAJ5",
        112500.0,
        "CAJ5",
    ),
    (
        "SOL-002",
        "Electrodo E6013 de 1/8\" (Suelto)",
        "KG",
        "KG",
        24000.0,
        "KG",
    ),
    (
        "SOL-003",
        "Electrodo E7018 de 1/8\" (Alta resistencia)",
        "KG",
        "CAJ5",
        130000.0,
        "CAJ5",
    ),
    (
        "PIN-001",
        "Thinner Corriente / Acrilico",
        "GAL",
        "GAL",
        32000.0,
        "GAL",
    ),
    (
        "PIN-002",
        "Pintura Anticorrosiva (Gris/Rojo)",
        "GAL",
        "GAL",
        57500.0,
        "GAL",
    ),
    (
        "PIN-003",
        "Laca / Esmalte Sintetico (Colores)",
        "GAL",
        "GAL",
        120000.0,
        "GAL",
    ),
    (
        "CON-001",
        "Disco de Corte para Metal 4.5\"",
        "UND",
        "UND",
        5500.0,
        "UND",
    ),
    (
        "CON-002",
        "Disco de Desbaste / Pulir 4.5\"",
        "UND",
        "UND",
        7500.0,
        "UND",
    ),
    (
        "CON-003",
        "Bisagra de Capsula o Torneada 5/8\"",
        "UND",
        "UND",
        4000.0,
        "UND",
    ),
];

const SEEDED_CONVERSIONS: [(&str, &str, f64); 11] = [
    ("KG", "G", 1000.0),
    ("L", "ML", 1000.0),
    ("GAL", "L", 3.78541),
    ("M", "CM", 100.0),
    ("CM", "MM", 10.0),
    ("VAR6", "M", 6.0),
    ("PLA6", "M", 6.0),
    ("TUB6", "M", 6.0),
    ("VAR6", "UND", 1.0),
    ("PLA6", "UND", 1.0),
    ("TUB6", "UND", 1.0),
];

pub async fn initialize(pool: &SqlitePool) -> AppResult<()> {
    for statement in SCHEMA_STATEMENTS {
        sqlx::query(statement).execute(pool).await?;
    }

    for (code, name, symbol) in DEFAULT_UNITS {
        sqlx::query(
            r#"
            INSERT OR IGNORE INTO units (code, name, symbol)
            VALUES (?, ?, ?)
            "#,
        )
        .bind(code)
        .bind(name)
        .bind(symbol)
        .execute(pool)
        .await?;
    }

    seed_material_catalog(pool).await?;
    seed_unit_conversions(pool).await?;

    ensure_finished_product_columns(pool).await?;
    ensure_recipe_item_columns(pool).await?;
    ensure_recipe_output_columns(pool).await?;
    ensure_recipe_migration(pool).await?;

    Ok(())
}

async fn seed_material_catalog(pool: &SqlitePool) -> AppResult<()> {
    for (code, name, symbol) in SEEDED_UNITS {
        sqlx::query(
            r#"
            INSERT OR IGNORE INTO units (code, name, symbol)
            VALUES (?, ?, ?)
            "#,
        )
        .bind(code)
        .bind(name)
        .bind(symbol)
        .execute(pool)
        .await?;
    }

    for (code, name, stock_unit_code, purchase_unit_code, unit_cost, cost_unit_code) in SEEDED_MATERIALS {
        let stock_unit_id = find_unit_id(pool, stock_unit_code).await?;
        let purchase_unit_id = find_unit_id(pool, purchase_unit_code).await?;
        let cost_unit_id = find_unit_id(pool, cost_unit_code).await?;

        let mut transaction = pool.begin().await?;

        let insert_result = sqlx::query(
            r#"
            INSERT OR IGNORE INTO materials (code, name, stock_unit_id, purchase_unit_id)
            VALUES (?, ?, ?, ?)
            "#,
        )
        .bind(code)
        .bind(name)
        .bind(stock_unit_id)
        .bind(purchase_unit_id)
        .execute(&mut *transaction)
        .await?;

        let material_id = if insert_result.rows_affected() > 0 {
            insert_result.last_insert_rowid()
        } else {
            sqlx::query_scalar::<_, i64>("SELECT id FROM materials WHERE code = ?")
                .bind(code)
                .fetch_one(&mut *transaction)
                .await?
        };

        let has_cost_history = sqlx::query_scalar::<_, i64>(
            "SELECT EXISTS(SELECT 1 FROM material_cost_history WHERE material_id = ?)",
        )
        .bind(material_id)
        .fetch_one(&mut *transaction)
        .await?;

        if has_cost_history == 0 {
            sqlx::query(
                r#"
                INSERT INTO material_cost_history (material_id, unit_id, unit_cost)
                VALUES (?, ?, ?)
                "#,
            )
            .bind(material_id)
            .bind(cost_unit_id)
            .bind(unit_cost)
            .execute(&mut *transaction)
            .await?;
        }

        transaction.commit().await?;
    }

    Ok(())
}

async fn find_unit_id(pool: &SqlitePool, code: &str) -> AppResult<i64> {
    Ok(sqlx::query_scalar::<_, i64>("SELECT id FROM units WHERE code = ?")
        .bind(code)
        .fetch_one(pool)
        .await?)
}

async fn seed_unit_conversions(pool: &SqlitePool) -> AppResult<()> {
    for (from_code, to_code, factor) in SEEDED_CONVERSIONS {
        let from_unit_id = find_unit_id(pool, from_code).await?;
        let to_unit_id = find_unit_id(pool, to_code).await?;

        sqlx::query(
            r#"
            INSERT INTO unit_conversions (from_unit_id, to_unit_id, factor)
            VALUES (?, ?, ?)
            ON CONFLICT (from_unit_id, to_unit_id)
            DO UPDATE SET factor = excluded.factor
            "#,
        )
        .bind(from_unit_id)
        .bind(to_unit_id)
        .bind(factor)
        .execute(pool)
        .await?;
    }

    Ok(())
}

async fn ensure_finished_product_columns(pool: &SqlitePool) -> AppResult<()> {
    let columns = sqlx::query("PRAGMA table_info(finished_products)")
        .fetch_all(pool)
        .await?;

    let has_category = columns.iter().any(|row| row.get::<String, _>("name") == "category");
    let has_status = columns.iter().any(|row| row.get::<String, _>("name") == "status");
    let has_recipe_id = columns.iter().any(|row| row.get::<String, _>("name") == "recipe_id");
    let has_internal_code = columns
        .iter()
        .any(|row| row.get::<String, _>("name") == "internal_code");
    let has_commercial_code = columns
        .iter()
        .any(|row| row.get::<String, _>("name") == "commercial_code");

    if !has_category {
        sqlx::query(
            "ALTER TABLE finished_products ADD COLUMN category TEXT NOT NULL DEFAULT 'General'",
        )
        .execute(pool)
        .await?;
    }

    if !has_status {
        sqlx::query(
            "ALTER TABLE finished_products ADD COLUMN status TEXT NOT NULL DEFAULT 'active'",
        )
        .execute(pool)
        .await?;
    }

    if !has_recipe_id {
        sqlx::query("ALTER TABLE finished_products ADD COLUMN recipe_id INTEGER REFERENCES recipes(id)")
            .execute(pool)
            .await?;
    }

    if !has_internal_code {
        sqlx::query("ALTER TABLE finished_products ADD COLUMN internal_code TEXT")
            .execute(pool)
            .await?;
    }

    if !has_commercial_code {
        sqlx::query("ALTER TABLE finished_products ADD COLUMN commercial_code TEXT")
            .execute(pool)
            .await?;
    }

    Ok(())
}

async fn ensure_recipe_migration(pool: &SqlitePool) -> AppResult<()> {
    let product_rows = sqlx::query(
        r#"
        SELECT id, code, name
        FROM finished_products
        WHERE recipe_id IS NULL
          AND EXISTS(SELECT 1 FROM product_bom_items WHERE product_id = finished_products.id)
        "#,
    )
    .fetch_all(pool)
    .await?;

    for row in product_rows {
        let product_id = row.get::<i64, _>("id");
        let product_code = row.get::<String, _>("code");
        let product_name = row.get::<String, _>("name");

        let mut transaction = pool.begin().await?;

        let recipe_result = sqlx::query(
            r#"
            INSERT INTO recipes (code, name, product_id, status, version, output_quantity, output_unit_id, notes)
            VALUES (?, ?, ?, 'active', 'v1', 1, (SELECT unit_id FROM finished_products WHERE id = ?), 'Migrada automaticamente desde BOM historica')
            "#,
        )
        .bind(format!("REC-{}", product_code))
        .bind(format!("Receta {}", product_name))
        .bind(product_id)
        .bind(product_id)
        .execute(&mut *transaction)
        .await?;

        let recipe_id = recipe_result.last_insert_rowid();

        sqlx::query(
            r#"
            INSERT INTO recipe_items (recipe_id, material_id, quantity, unit_id)
            SELECT ?, pbi.material_id, pbi.quantity, m.stock_unit_id
            FROM product_bom_items pbi
            INNER JOIN materials m ON m.id = pbi.material_id
            WHERE pbi.product_id = ?
            "#,
        )
        .bind(recipe_id)
        .bind(product_id)
        .execute(&mut *transaction)
        .await?;

        sqlx::query("UPDATE finished_products SET recipe_id = ? WHERE id = ?")
            .bind(recipe_id)
            .bind(product_id)
            .execute(&mut *transaction)
            .await?;

        transaction.commit().await?;
    }

    Ok(())
}

async fn ensure_recipe_item_columns(pool: &SqlitePool) -> AppResult<()> {
    let columns = sqlx::query("PRAGMA table_info(recipe_items)")
        .fetch_all(pool)
        .await?;

    let has_unit_id = columns.iter().any(|row| row.get::<String, _>("name") == "unit_id");

    if !has_unit_id {
        sqlx::query("ALTER TABLE recipe_items ADD COLUMN unit_id INTEGER REFERENCES units(id)")
            .execute(pool)
            .await?;

        sqlx::query(
            r#"
            UPDATE recipe_items
            SET unit_id = (
              SELECT materials.stock_unit_id
              FROM materials
              WHERE materials.id = recipe_items.material_id
            )
            WHERE unit_id IS NULL
            "#,
        )
        .execute(pool)
        .await?;
    }

    Ok(())
}

async fn ensure_recipe_output_columns(pool: &SqlitePool) -> AppResult<()> {
    let columns = sqlx::query("PRAGMA table_info(recipes)")
        .fetch_all(pool)
        .await?;

    let has_output_quantity = columns
        .iter()
        .any(|row| row.get::<String, _>("name") == "output_quantity");
    let has_output_unit_id = columns
        .iter()
        .any(|row| row.get::<String, _>("name") == "output_unit_id");

    if !has_output_quantity {
        sqlx::query("ALTER TABLE recipes ADD COLUMN output_quantity REAL NOT NULL DEFAULT 1")
            .execute(pool)
            .await?;
    }

    if !has_output_unit_id {
        sqlx::query("ALTER TABLE recipes ADD COLUMN output_unit_id INTEGER REFERENCES units(id)")
            .execute(pool)
            .await?;

        sqlx::query(
            r#"
            UPDATE recipes
            SET output_unit_id = (
              SELECT finished_products.unit_id
              FROM finished_products
              WHERE finished_products.id = recipes.product_id
            )
            WHERE output_unit_id IS NULL AND product_id IS NOT NULL
            "#,
        )
        .execute(pool)
        .await?;

        sqlx::query("UPDATE recipes SET output_unit_id = 5 WHERE output_unit_id IS NULL")
            .execute(pool)
            .await?;
    }

    Ok(())
}

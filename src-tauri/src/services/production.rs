use crate::state::AppState;

pub struct ProductionService<'a> {
    _state: &'a AppState,
}

#[derive(Clone)]
pub struct ProductionTab {
    pub key: String,
    pub label: String,
    pub icon: String,
    pub route: String,
}

impl<'a> ProductionService<'a> {
    pub fn new(state: &'a AppState) -> Self {
        Self { _state: state }
    }

    pub fn list_tabs(&self) -> Vec<ProductionTab> {
        vec![
            ProductionTab {
                key: String::from("unidades"),
                label: String::from("Unidades"),
                icon: String::from("mdi-ruler-square"),
                route: String::from("/produccion/unidades"),
            },
            ProductionTab {
                key: String::from("recetas"),
                label: String::from("Recetas"),
                icon: String::from("mdi-notebook-edit-outline"),
                route: String::from("/produccion/recetas"),
            },
            ProductionTab {
                key: String::from("productos"),
                label: String::from("Productos"),
                icon: String::from("mdi-package-variant-closed"),
                route: String::from("/produccion/productos"),
            },
        ]
    }
}

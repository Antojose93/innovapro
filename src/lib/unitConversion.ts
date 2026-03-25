import type { UnitConversion } from '@/types/masterData'

export const resolveConversionFactor = (
  fromUnitId: number,
  toUnitId: number,
  conversions: UnitConversion[]
): number => {
  if (fromUnitId === toUnitId) {
    return 1
  }

  const graph = new Map<number, Array<{ to: number; factor: number }>>()

  for (const conversion of conversions) {
    graph.set(conversion.fromUnitId, [
      ...(graph.get(conversion.fromUnitId) ?? []),
      { to: conversion.toUnitId, factor: conversion.factor },
    ])
    graph.set(conversion.toUnitId, [
      ...(graph.get(conversion.toUnitId) ?? []),
      { to: conversion.fromUnitId, factor: 1 / conversion.factor },
    ])
  }

  const queue: Array<{ unitId: number; factor: number }> = [{ unitId: fromUnitId, factor: 1 }]
  const visited = new Set<number>([fromUnitId])

  while (queue.length > 0) {
    const current = queue.shift()

    if (!current) {
      break
    }

    if (current.unitId === toUnitId) {
      return current.factor
    }

    for (const neighbor of graph.get(current.unitId) ?? []) {
      if (!visited.has(neighbor.to)) {
        visited.add(neighbor.to)
        queue.push({
          unitId: neighbor.to,
          factor: current.factor * neighbor.factor,
        })
      }
    }
  }

  throw new Error('No existe una conversion registrada entre esas unidades')
}

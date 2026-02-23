/**
 * Maps DSP item numeric IDs to their icon filenames (without .png extension)
 * and Chinese display names. Derived from dsp_blueprint_editor itemsData.ts.
 */
export interface ItemMeta {
  icon: string
  name: string
}

export const ITEM_ICONS: Record<number, ItemMeta> = {
  // Raw resources
  1001: { icon: 'iron-ore',        name: '铁矿' },
  1002: { icon: 'copper-ore',      name: '铜矿' },
  1003: { icon: 'silicium-ore',    name: '硅石' },
  1004: { icon: 'titanium-ore',    name: '钛石' },
  1005: { icon: 'stone-ore',       name: '石矿' },
  1006: { icon: 'coal-ore',        name: '煤矿' },
  1007: { icon: 'oil',             name: '原油' },
  1000: { icon: 'water',           name: '水' },
  1011: { icon: 'gas-hydrate',     name: '可燃冰' },
  1012: { icon: 'diamond-ore',     name: '金伯利矿石' },
  1013: { icon: 'fractal-silica',  name: '分形硅石' },
  1014: { icon: 'grating-ore',     name: '光栅石' },
  1015: { icon: 'bamboo-crystal',  name: '刺笋结晶' },
  1016: { icon: 'mono-mag-ore',    name: '单极磁石' },
  1030: { icon: 'wood',            name: '木材' },
  1031: { icon: 'plant-fuel',      name: '植物燃料' },

  // Smelted / processed materials
  1101: { icon: 'iron-plate',              name: '铁块' },
  1102: { icon: 'magnet',                  name: '磁铁' },
  1103: { icon: 'steel-plate',             name: '钢材' },
  1104: { icon: 'copper-plate',            name: '铜块' },
  1105: { icon: 'silicium-single-crystal', name: '高纯硅块' },
  1106: { icon: 'titanium-plate',          name: '钛块' },
  1107: { icon: 'titanium-alloy',          name: '钛合金' },
  1108: { icon: 'stone-brick',             name: '石材' },
  1109: { icon: 'graphite',                name: '高能石墨' },
  1110: { icon: 'glass',                   name: '玻璃' },
  1111: { icon: 'prism',                   name: '棱镜' },
  1112: { icon: 'diamond',                 name: '金刚石' },
  1113: { icon: 'silicium-high-purity',    name: '晶格硅' },
  1114: { icon: 'refined-oil',             name: '精炼油' },
  1115: { icon: 'plastic',                 name: '塑料' },
  1116: { icon: 'sulphuric-acid',          name: '硫酸' },
  1117: { icon: 'crystal-rubber',          name: '有机晶体' },
  1118: { icon: 'titan-crystal',           name: '钛晶石' },
  1119: { icon: 'titan-glass',             name: '钛化玻璃' },
  1120: { icon: 'hydrogen',               name: '氢' },
  1121: { icon: 'deuterium',              name: '重氢' },
  1122: { icon: 'anti-matter',            name: '反物质' },
  1123: { icon: 'graphene',              name: '石墨烯' },
  1124: { icon: 'nanotube',             name: '碳纳米管' },
  1125: { icon: 'frame-material',       name: '框架材料' },
  1126: { icon: 'casimir-crystal',      name: '卡西米尔晶体' },
  1127: { icon: 'strange-matter-generator', name: '奇异物质' },
  1128: { icon: 'explosive',            name: '燃烧单元' },
  1129: { icon: 'high-explosive',       name: '爆破单元' },
  1130: { icon: 'particle-explosive',   name: '晶石爆破单元' },
  1131: { icon: 'terrain-tool',         name: '地基' },
  1141: { icon: 'accelerator-1',        name: '增产剂 Mk.I' },
  1142: { icon: 'accelerator-2',        name: '增产剂 Mk.II' },
  1143: { icon: 'accelerator-3',        name: '增产剂 Mk.III' },

  // Components
  1201: { icon: 'gear-wheel',           name: '齿轮' },
  1202: { icon: 'magnetism-wire',       name: '磁线圈' },
  1203: { icon: 'electric-motor',       name: '电动机' },
  1204: { icon: 'mag-turbine',          name: '电磁涡轮' },
  1205: { icon: 'hyper-magnetism-ring', name: '超级磁场环' },
  1206: { icon: 'partical-capacitor',   name: '粒子容器' },
  1208: { icon: 'photon-capacitor-full',name: '临界光子' },
  1209: { icon: 'gravity-lens',         name: '引力透镜' },
  1210: { icon: 'space-warper',         name: '空间翘曲器' },
  1301: { icon: 'circuit-board',        name: '电路板' },
  1302: { icon: 'micro-component',      name: '微晶元件' },
  1303: { icon: 'processor',            name: '处理器' },
  1304: { icon: 'plane-filter',         name: '位面过滤器' },
  1305: { icon: 'quantum-processor',    name: '量子芯片' },
  1401: { icon: 'plasma-generator',     name: '电浆激发器' },
  1402: { icon: 'particle-wide-band',   name: '粒子宽带' },
  1403: { icon: 'fusion-capacitor',     name: '湮灭约束球' },
  1404: { icon: 'photo-shifter',        name: '光子合并器' },
  1405: { icon: 'fuel-thruster',        name: '推进器' },
  1406: { icon: 'ion-thruster',         name: '加力推进器' },
  1407: { icon: 'engine',              name: '动力引擎' },

  // Fuel rods
  1801: { icon: 'hydrogen-energy-fuel',   name: '氢燃料棒' },
  1802: { icon: 'deuterium-energy-fuel',  name: '氘核燃料棒' },
  1803: { icon: 'antimatter-energy-fuel', name: '反物质燃料棒' },
  1804: { icon: 'virtual-energy-fuel',    name: '金色燃料棒' },

  // Dyson sphere materials
  1501: { icon: 'solar-collector',       name: '太阳帆' },
  1502: { icon: 'dyson-sphere-component',name: '戴森球组件' },
  1503: { icon: 'rocket',               name: '小型运载火箭' },

  // Negentropy items
  5201: { icon: 'memory',           name: '存储单元' },
  5202: { icon: 'silicon-neuron',   name: '硅基神经元' },
  5203: { icon: 'reassembler',      name: '物质重组器' },
  5204: { icon: 'negentropy',       name: '负熵奇点' },
  5205: { icon: 'virtual-particle', name: '虚粒子' },
  5206: { icon: 'energy-fragment',  name: '能量碎片' },

  // Matrix science
  6001: { icon: 't-matrix', name: '电磁矩阵' },
  6002: { icon: 'e-matrix', name: '能量矩阵' },
  6003: { icon: 'c-matrix', name: '结构矩阵' },
  6004: { icon: 'i-matrix', name: '信息矩阵' },
  6005: { icon: 'g-matrix', name: '引力矩阵' },
  6006: { icon: 'u-matrix', name: '宇宙矩阵' },

  // Military items
  1601: { icon: 'bullet',           name: '机枪弹箱' },
  1602: { icon: 'bullet-titanium',  name: '钛化弹箱' },
  1603: { icon: 'bullet-alloy',     name: '超合金弹箱' },
  1604: { icon: 'cannonball',       name: '炮弹组' },
  1605: { icon: 'cannonball-cluster',name: '高爆炮弹组' },
  1606: { icon: 'cannonball-alloy', name: '晶石炮弹组' },
  1607: { icon: 'capsule-energy',   name: '等离子胶囊' },
  1608: { icon: 'capsule-antimatter',name: '反物质胶囊' },
  1609: { icon: 'missile',          name: '导弹组' },
  1610: { icon: 'missile-supersonic',name: '超音速导弹组' },
  1611: { icon: 'missile-graviton', name: '引力导弹组' },
  1612: { icon: 'em-disturb',       name: '电磁干扰胶囊' },
  1613: { icon: 'em-suppress',      name: '电磁压制胶囊' },
  5101: { icon: 'fighter-shield',   name: '地面战斗机-E型' },
  5102: { icon: 'fighter-plasma',   name: '地面战斗机-A型' },
  5103: { icon: 'fighter-laser',    name: '地面战斗机-F型' },
  5111: { icon: 'warship-plasma',   name: '太空战斗机-A型' },
  5112: { icon: 'warship-laser',    name: '太空战斗机-F型' },

  // Logistics items
  5001: { icon: 'logistic-drone',   name: '物流运输机' },
  5002: { icon: 'logistic-vessel',  name: '星际物流运输船' },
  5003: { icon: 'delivery-drone',   name: '配送运输机' },

  // --- Buildings ---
  // Conveyor belts
  2001: { icon: 'belt-1', name: '低速传送带' },
  2002: { icon: 'belt-2', name: '高速传送带' },
  2003: { icon: 'belt-3', name: '极速传送带' },
  // Sorters
  2011: { icon: 'inserter-1', name: '低速分拣器' },
  2012: { icon: 'inserter-2', name: '高速分拣器' },
  2013: { icon: 'inserter-3', name: '极速分拣器' },
  2014: { icon: 'inserter-4', name: '集装分拣器' },
  // Splitter / piler / monitor
  2020: { icon: 'splitter-4dir', name: '四向分流器' },
  2030: { icon: 'monitor',       name: '流速器' },
  2040: { icon: 'piler',         name: '自动集装机' },
  2313: { icon: 'spray-coater',  name: '喷涂机' },
  // Storage
  2101: { icon: 'storage-1',    name: '小型储物仓' },
  2102: { icon: 'storage-2',    name: '大型储物仓' },
  2106: { icon: 'storage-tank', name: '储液罐' },
  // Logistics
  2103: { icon: 'logistic-station',              name: '物流运输站' },
  2104: { icon: 'interstellar-logistic-station', name: '星际物流运输站' },
  2105: { icon: 'orbital-collector',             name: '轨道采集器' },
  2107: { icon: 'delivery-machine',              name: '物流配送器' },
  // Assemblers
  2303: { icon: 'assembler-1', name: '制造台 Mk.I' },
  2304: { icon: 'assembler-2', name: '制造台 Mk.II' },
  2305: { icon: 'assembler-3', name: '制造台 Mk.III' },
  2318: { icon: 'assembler-4', name: '制造台 Mk.IV' },
  // Smelters
  2302: { icon: 'smelter',   name: '电弧熔炉' },
  2315: { icon: 'smelter-2', name: '位面熔炉' },
  2319: { icon: 'smelter-3', name: '熔炉 Mk.III' },
  // Power
  2201: { icon: 'tesla-coil',             name: '电力感应塔' },
  2202: { icon: 'charging-pole',           name: '无线输电塔' },
  2203: { icon: 'wind-turbine',            name: '风力涡轮机' },
  2204: { icon: 'fuel-plant',              name: '火力发电厂' },
  2205: { icon: 'solar-panel',             name: '太阳能板' },
  2206: { icon: 'accumulator',             name: '蓄电器' },
  2207: { icon: 'accumulator-full',        name: '蓄电器（满）' },
  2209: { icon: 'energy-exchanger',        name: '能量枢纽' },
  2210: { icon: 'fusion-reactor',          name: '人造恒星' },
  2211: { icon: 'fusion-power-station',    name: '微型聚变发电站' },
  2212: { icon: 'orbital-substation',      name: '卫星配电站' },
  2213: { icon: 'geothermal-power-station',name: '地热发电站' },
  // Mining / extraction
  2301: { icon: 'mining-drill',    name: '采矿机' },
  2306: { icon: 'water-pump',      name: '抽水站' },
  2307: { icon: 'oil-extractor',   name: '原油萃取站' },
  2316: { icon: 'mining-drill-mk2',name: '大型采矿机' },
  // Processing
  2308: { icon: 'oil-refinery',      name: '原油精炼厂' },
  2309: { icon: 'chemical-plant',    name: '化工厂' },
  2314: { icon: 'fractionator',      name: '分馏塔' },
  2317: { icon: 'chemical-plant-2',  name: '化工厂 Mk.II' },
  // Labs
  2901: { icon: 'lab',   name: '矩阵研究站' },
  2902: { icon: 'lab-2', name: '矩阵研究站 Mk.II' },
  // Dyson sphere structures
  2208: { icon: 'ray-receiver',        name: '射线接收站' },
  2311: { icon: 'em-rail-ejector',     name: '电磁轨道弹射器' },
  2312: { icon: 'vertical-launching-silo', name: '垂直发射井' },
  // Hadron collider
  2310: { icon: 'hadron-collider', name: '微型粒子对撞机' },
  // Military buildings
  3001: { icon: 'turret-gauss',         name: '高斯机枪塔' },
  3002: { icon: 'turret-laser',         name: '高频激光塔' },
  3003: { icon: 'turret-cannon',        name: '聚爆加农炮' },
  3004: { icon: 'turret-plasma',        name: '磁化电浆炮' },
  3005: { icon: 'turret-missile',       name: '导弹防御塔' },
  3006: { icon: 'turret-disturb',       name: '干扰塔' },
  3007: { icon: 'turret-signal',        name: '信标' },
  3008: { icon: 'turret-shield',        name: '护盾发生器' },
  3009: { icon: 'battle-base',          name: '战场分析基站' },
  3010: { icon: 'turret-plasma-ground', name: '地面电浆炮' },

  // Soil pile
  1099: { icon: 'soil-pile', name: '沙土' },
}

/**
 * Returns the icon URL for a given item ID.
 * Icons are served from /icons/item_recipe/ (the public folder).
 * Falls back to undefined if the item ID is not known.
 */
export function getItemIconUrl(itemId: number): string | undefined {
  const meta = ITEM_ICONS[itemId]
  if (!meta) return undefined
  return `${import.meta.env.BASE_URL}icons/item_recipe/${meta.icon}.png`
}

export function getItemName(itemId: number): string {
  return ITEM_ICONS[itemId]?.name ?? `Item #${itemId}`
}

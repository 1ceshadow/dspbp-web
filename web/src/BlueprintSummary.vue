<template>
  <div v-if="items.length > 0" class="bp-summary">
    <p class="bp-summary-title">
      包含设施 · 共 <strong>{{ totalCount }}</strong> 个
    </p>
    <div class="bp-summary-icons">
      <div
        v-for="item in items"
        :key="item.id"
        class="bp-icon-cell"
        :title="`${getItemName(item.id)} × ${item.count}`"
      >
        <img
          v-if="getItemIconUrl(item.id)"
          :src="getItemIconUrl(item.id)"
          :alt="getItemName(item.id)"
          class="bp-item-icon"
          loading="lazy"
        />
        <div v-else class="bp-item-icon bp-item-icon-fallback">
          {{ item.id }}
        </div>
        <span class="bp-item-count">{{ item.count }}</span>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { getItemIconUrl, getItemName } from './itemIcons'

export interface BuildingCount {
  id: number
  count: number
}

const props = defineProps<{
  items: BuildingCount[]
}>()

const totalCount = computed(() =>
  props.items.reduce((sum, i) => sum + i.count, 0)
)
</script>

<style scoped>
.bp-summary {
  margin-top: 4px;
  padding: 10px 12px;
  background: var(--bg-input);
  border: 1px solid var(--border-color);
  border-radius: 8px;
  box-shadow: inset 0 1px 3px rgba(0,0,0,0.1);
}

.bp-summary-title {
  margin: 0 0 10px;
  font-size: 0.75rem;
  color: var(--text-secondary);
  text-transform: uppercase;
  letter-spacing: 0.06em;
  font-weight: 600;
}

.bp-summary-title strong {
  color: var(--text-primary);
  font-variant-numeric: tabular-nums;
}

.bp-summary-icons {
  display: flex;
  flex-wrap: wrap;
  gap: 6px;
}

.bp-icon-cell {
  display: flex;
  flex-direction: column;
  align-items: center;
  width: 46px;
  cursor: default;
  transition: transform 0.15s;
}

.bp-icon-cell:hover {
  transform: translateY(-2px);
}

.bp-item-icon {
  width: 38px;
  height: 38px;
  object-fit: contain;
  display: block;
  image-rendering: pixelated;
  border-radius: 4px;
}

.bp-item-icon-fallback {
  width: 38px;
  height: 38px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: var(--bg-select);
  border: 1px solid var(--border-color);
  border-radius: 4px;
  font-size: 0.6rem;
  color: var(--text-secondary);
  text-align: center;
  word-break: break-all;
}

.bp-item-count {
  font-size: 0.72rem;
  font-weight: 600;
  color: var(--count-color);
  text-shadow: var(--count-shadow);
  margin-top: 3px;
  line-height: 1;
  font-variant-numeric: tabular-nums;
}
</style>

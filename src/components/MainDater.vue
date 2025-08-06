<template>
    <div tabindex="0"  :class="['collapse', 'max-w-md', 'text-center', { 'collapse-open' : isOpen }, 'bg-base-100', 'border-base-300', 'border']" @click="isOpen = !isOpen">
        <div class="collapse-title"> 
            <h1 class="text-4xl">{{ props.currentMonth.label }}</h1>
            </div>
        <div class="collapse-content grid-container">
            <button
                class="hover:bg-indigo-100 transition-colors grid-item" 
                v-for="month in months" 
                :key="month.value" 
                @click.stop="updateMonth(month)"
            >{{ month.label }}</button>
        </div>
    </div>
</template>
<script setup lang="ts">
import { ref } from 'vue'
import 'cally'
import type { ActivityDate, DateRange } from '../models/ActivityDate'
import { utils } from '../shared/utils';

const props = defineProps<{ currentMonth: ActivityDate }>()
const emit = defineEmits<{(e: 'month-changed', currentRange: DateRange): void}>()

const months = utils.getMonthsList()
const isOpen = ref<boolean>(false)

function updateMonth(monthValue : ActivityDate){
    console.log("monthValue: ", monthValue)
    const currentRange = utils.getMonthInitialFinal(monthValue.value, 2025);
    emit('month-changed', currentRange)
    isOpen.value = false
}
</script>
<style lang="sass">
.grid-container
    display: grid
    grid-template-columns: repeat(3, 1fr)
    gap: 5px

.grid-item
    padding: 16px
    border-radius: 4px
</style>
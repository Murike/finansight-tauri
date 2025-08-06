<template>
    <div class="grid grid-cols-12 gap-4">
        <div class="col-span-4"></div>
        <div class="col-span-4 flex flex-col items-center gap-4">
            <MainDater :currentMonth @month-changed="onMonthChanged"></MainDater>
            <div class="overflow-auto" v-for="activity of activities">
                <ActivityConsolidated :data="activity" @delete-activity="deleteActivity"></ActivityConsolidated>
            </div>
        </div>
        <div class="col-span-4 ml-auto">
            <activity @activity-created="loadActivities"></activity>
        </div>
    </div>
</template>
<script setup lang="ts">
import {ref, onMounted, computed } from 'vue'
import MainDater from "../../components/MainDater.vue"
import activity from './Activity.vue'
import ActivityService from '../../services/ActivityService'
import type { Activity } from '../../models/Activity'
import type { DateRange } from '../../models/ActivityDate'
import ActivityConsolidated from './ActivityConsolidated.vue'
import { utils } from '../../shared/utils'
import dayjs from 'dayjs';

const activities = ref<Activity[]>([])
const appliedDateRange = ref<DateRange>(utils.getMonthInitialFinal());
const currentMonth = computed(() => utils.getMonthsList()[dayjs.utc(appliedDateRange.value.initialDate, undefined, true).month()])

onMounted(async () => {
    await loadActivities();
})

const onMonthChanged = async(monthValue: DateRange) => {
    appliedDateRange.value = monthValue;
    await loadActivities();
}

const loadActivities = async() => {
    activities.value = await ActivityService.getInstance().getActivities(appliedDateRange.value);
}

const deleteActivity = async(activityId : string) => {
    const rowsUpdated = await ActivityService.getInstance().deleteActivity(activityId);
    console.log("rowsUpdated: ", rowsUpdated)
    if(rowsUpdated > 0)
        await loadActivities();
}
</script>
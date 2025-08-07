<template>
    <div class="grid grid-cols-12 gap-4 h-full overflow-hidden">
        <div class="col-span-4 flex flex-col h-full overflow-hidden">
            <div class="flex-1 flex flex-col bg-gray-700 p-4 rounded min-h-0 overflow-auto custom-scrollbar">
                <div class="flex-shrink-0 flex">
                    <h3 class="text-lg font-semibold mb-4">Filter 1</h3>
                </div>
                <div class="flex-1 overflow-auto custom-scrollbar">
                    <div class="flex flex-col gap-4 py-4 px-2">
                        <ActivityConsolidated 
                            class="w-full"
                            v-for="activity of activities" 
                            :key="activity.id"
                            :data="activity" 
                            @delete-activity="deleteActivity"
                            @tag-clicked="removeFilterTag">
                        </ActivityConsolidated>
                    </div>
                </div>
                <div class="flex-shrink-0 flex">
                    <h3 class="text-lg font-semibold ">Bottom, extra data</h3>
                </div>
            </div>
            
            <div class="divider my-2"></div>
            
            <div class="flex-1 bg-gray-700 p-4 rounded min-h-0 overflow-auto custom-scrollbar">
                <h3 class="text-lg font-semibold mb-4">Filter 2</h3>
                <!-- Even if this is empty, the div still has height due to flex-1 -->
            </div>
        </div>
        <div class="col-span-4 flex flex-col h-full overflow-hidden">
            <div class="flex-shrink-0 flex justify-center pb-8">
                <MainDater :currentMonth @month-changed="onMonthChanged"></MainDater>
            </div>
            <div class="flex-1 overflow-auto custom-scrollbar">
                <div class="flex flex-col gap-4 py-4 px-2">
                    <ActivityConsolidated 
                        class="w-full"
                        v-for="activity of activities" 
                        :key="activity.id"
                        :data="activity" 
                        @delete-activity="deleteActivity">
                    </ActivityConsolidated>
                </div>
            </div>
        </div>
        <div class="col-span-4 ml-auto h-full overflow-auto custom-scrollbar">
            <activity @activity-created="loadActivities"></activity>
        </div>
    </div>
</template>
<script setup lang="ts">
import {ref, onMounted, computed } from 'vue'
import MainDater from "../../components/MainDater.vue"
import activity from './Activity.vue'
import ActivityService from '../../services/ActivityService'
import type { Activity, Tag } from '../../models/Activity'
import type { DateRange } from '../../models/ActivityDate'
import ActivityConsolidated from './ActivityConsolidated.vue'
import { utils } from '../../shared/utils'
import dayjs from 'dayjs';

const activities = ref<Activity[]>([])
const appliedDateRange = ref<DateRange>(utils.getMonthInitialFinal());
const currentMonth = computed(() => utils.getMonthsList()[dayjs.utc(appliedDateRange.value.initialDate, undefined, true).month()])

const createDefaultFilter = () => {
    return {
        id : '',
        description: '',
        initialDate: '',
        finalDate: ''  
    }
}

onMounted(async () => {
    await loadActivities();
})

const onMonthChanged = async(monthValue: DateRange) => {
    appliedDateRange.value = monthValue;
    await loadActivities();
}

const loadActivities = async() => {
    activities.value = await ActivityService.getInstance().getActivities(appliedDateRange.value);
    // load filters list
    // load filters activities
}

const deleteActivity = async(activityId : string) => {
    const rowsUpdated = await ActivityService.getInstance().deleteActivity(activityId);
    if(rowsUpdated > 0)
        await loadActivities();
}

const addFilterTag = async(tag : Tag) => {
    // UPDATE FILTER TO ADD TAG
    // RELOAD FILTER DATA

    // const rowsUpdated = await ActivityService.getInstance().deleteActivity(activityId);
    // if(rowsUpdated > 0)
    //     await loadActivities();
}

const removeFilterTag = async(tag : Tag) => {
    console.log("tag: ", tag)
    // UPDATE FILTER TO REMOVE TAG
    // RELOAD FILTER DATA

    // const rowsUpdated = await ActivityService.getInstance().deleteActivity(activityId);
    // if(rowsUpdated > 0)
    //     await loadActivities();
}

</script>
<style lang="sass" scoped>
.custom-scrollbar
    scrollbar-width: none
    -ms-overflow-style: none
    
    &::-webkit-scrollbar
        display: none
</style>
<template>
    <div class="grid grid-cols-12 gap-4 h-full overflow-hidden">
        <div class="col-span-4 flex flex-col h-full overflow-hidden">
            <div class="flex-1 flex flex-col bg-gray-700 p-4 rounded min-h-0 overflow-auto custom-scrollbar">
                <div v-if="leftFilterData.length > 0" class="flex-shrink-0 grid grid-cols-1 md:grid-cols-3 gap-4">
                    <h3 class="text-lg font-semibold mb-4">Filter 1</h3>
                    <input type="date" class="input w-full" placeholder="Data" v-model="leftFilterData[0].filter.initialDate">
                    <input type="date" class="input w-full" placeholder="Data" v-model="leftFilterData[0].filter.finalDate" @keydown.enter="updateFilterDaterange">
                </div>
                <div class="flex-1 overflow-auto custom-scrollbar">
                    <div class="flex flex-col gap-4 py-4 px-2">
                        <ActivityConsolidated 
                            v-if="leftFilterData.length > 0"
                            class="w-full"
                            v-for="activity of leftFilterData[0].activities" 
                            :key="activity.id"
                            :data="activity" 
                            @delete-activity="deleteActivity">
                        </ActivityConsolidated>
                    </div>
                </div>
                <div class="flex-shrink-0 grid grid-cols-12 gap-4 items-start">
                    <div v-if="leftFilterData.length > 0" class="col-span-8 overflow-y-auto flex flex-wrap gap-2">
                        <div v-for="tag in leftFilterData[0].filter.tags" class="badge cursor-pointer" @click="removeFilterTag(leftFilterData[0].filter.id, tag)">{{ tag.name }}</div>
                    </div>
                    <div class="col-span-4 text-right space-y-1">
                        <h2 class="font-semibold">R$ {{ leftFilterTotal }}</h2>
                    </div>
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
                        @delete-activity="deleteActivity"
                        @tag-clicked="addFilterTag">
                    </ActivityConsolidated>
                </div>
            </div>
        </div>
        <div class="col-span-4 ml-auto h-full overflow-auto custom-scrollbar">
            <activity @activity-created="loadActivities"></activity>
            <div class="py-12 flex flex-col"> 
                <div class="flex"> <h3>Créditos: {{ totalValues.positive }}</h3>  </div>
                <div class="flex"> <h3>Débitos: {{ totalValues.negative }}</h3> </div>
                <h2 class="flex py-12 text-2xl font-semibold">Total: {{ totalValues.total }}</h2>
            </div>
        </div>
    </div>
</template>
<script setup lang="ts">
import {ref, onMounted, computed } from 'vue'
import MainDater from "../../components/MainDater.vue"
import activity from './Activity.vue'
import ActivityService from '../../services/ActivityService'
import { Operation, type Activity, type Tag } from '../../models/Activity'
import type { DateRange } from '../../models/ActivityDate'
import ActivityConsolidated from './ActivityConsolidated.vue'
import { utils } from '../../shared/utils'
import dayjs from 'dayjs';
import { ActivityFilter, StaticFilter } from '../../models/Filters'

const activities = ref<Activity[]>([])
const filter1Activities = ref<Activity[]>([])
const appliedDateRange = ref<DateRange>(utils.getMonthInitialFinal());
const currentMonth = computed(() => utils.getMonthsList()[dayjs.utc(appliedDateRange.value.initialDate, undefined, true).month()])
const leftFilterData = ref<Array<{ activities : Activity[], filter: StaticFilter}>>([])
const totalValues = computed(() => { return activities.value.reduce((totals, activity) => {
    if(activity.operation == Operation.credit){
        totals.positive += activity.value;
        totals.total += activity.value;
    }else{
        totals.negative += activity.value;
        totals.total -= activity.value;
    }

    return totals;
}, {positive: 0, negative: 0, total: 0}) });
const leftFilterTotal = computed(() => { 
    
    if(leftFilterData.value.length > 0){
        return leftFilterData.value[0].activities.reduce((totals, activity) => {
        if(activity.operation == Operation.credit){
            totals += activity.value;
        }else{
            totals -= activity.value;
        }

        return totals;
    }, 0) }
    else return 0;
    }
);

const timelineFilter = computed(() => { return { initialDate : appliedDateRange.value.initialDate, finalDate: appliedDateRange.value.finalDate }});

onMounted(async () => {
    await loadActivities();
    await loadFilterActivities();
})

const onMonthChanged = async(monthValue: DateRange) => {
    appliedDateRange.value = monthValue;
    await loadActivities();
}

const loadActivities = async() => {
    activities.value = await ActivityService.getInstance().getActivities(timelineFilter.value);
}

const loadFilterActivities = async() => {
    const staticFilters = await ActivityService.getInstance().getStaticFilters();
    for(const filter of staticFilters){
        const activityFilter : ActivityFilter = {
            initialDate: filter.initialDate,
            finalDate: filter.finalDate,
            tags: filter.tags
        };

        filter1Activities.value = await ActivityService.getInstance().getActivities(activityFilter);
        if(leftFilterData.value[0])
            leftFilterData.value[0] = {activities: filter1Activities.value, filter : filter};
        else
            leftFilterData.value.push({activities: filter1Activities.value, filter : filter});
    }
}

const deleteActivity = async(activityId : string) => {
    await ActivityService.getInstance().deleteActivity(activityId);
    await loadActivities();
    await loadFilterActivities();
}

const removeFilterTag = async(filterId: string, tag : Tag) => {
    const updatedFilter = await ActivityService.getInstance().removeFilterTag(filterId, tag);
    leftFilterData.value[0].filter = updatedFilter;

    await upperLeftFilterUpdate();
}

const addFilterTag = async(tag : Tag) => {    
    const filterId = leftFilterData.value[0].filter.id;

    const updatedFilter = await ActivityService.getInstance().addFilterTag(filterId, tag);
    leftFilterData.value[0].filter = updatedFilter;

    await upperLeftFilterUpdate();
}

const updateFilterDaterange = async() => {    
    await ActivityService.getInstance().updateFilterDaterange(leftFilterData.value[0].filter);
    await upperLeftFilterUpdate();
}

const upperLeftFilterUpdate = async() => {    
    const updatedFilterActivities = await ActivityService.getInstance().getActivities(leftFilterData.value[0].filter);
    leftFilterData.value[0].activities = updatedFilterActivities;
}


</script>
<style lang="sass" scoped>
.custom-scrollbar
    scrollbar-width: none
    -ms-overflow-style: none
    
    &::-webkit-scrollbar
        display: none
</style>
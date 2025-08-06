<template>
    <div class="card w-160 pt-2">
        <div class="card-body p-1 flex flex-row gap-x-1">
            <div class="col">
                {{ dayjs(data.date).utc().format('DD') }}
            </div>
            <div class="col w-full">
                <div class="flex flex-row ">
                    <div class="indicator bg-gray-800 p-1" style="flex: 1;">
                        <span class="indicator-item badge p-0 size-6">
                            <TransactionDebit v-if="data.operation == Operation.debit" class="h-10 w-10"></TransactionDebit>
                            <TransactionCredit v-else class="h-10 w-10"></TransactionCredit>
                        </span>
                        <span class="label text-3xl font-bold">{{ data.value.toFixed(2) }}</span>
                    </div>
                    <div class="p-2 flex-4 flex-row bg-gray-900 join relative items-center overflow-hidden" >
                        <div class="label join-item text-gray-500 text-3xl truncate pr-2">{{ data.description }}</div>
                        <div class="label join-item text-gray-500 ml-auto whitespace-nowrap">{{ data.medium.name }}</div>
                        <button class="absolute bottom-0 right-0 btn bg-red-300 btn-xs p-0 min-h-0 h-3 w-3 rounded-full" @click="deleteActivity(data.id!)">
                            <span class="text-xs"></span>
                        </button>
                    </div>
                </div>
                <div class="col flex flex-row">
                    <div v-for="tag in data.tags" class="badge">{{ tag.name }}</div>
                </div>
            </div>
        </div>
    </div>
</template>
<script setup lang="ts">
import { TransactionCredit, TransactionDebit } from '../../components/icons';
import { Operation, type Activity } from '../../models/Activity';
import dayjs from 'dayjs';

const emit = defineEmits<{
    (e: 'delete-activity', activityId: string): void
}>();

defineProps<{
    data : Activity
}>()

const deleteActivity = async(activityId: string)=>{
    emit('delete-activity', activityId)
}
</script>
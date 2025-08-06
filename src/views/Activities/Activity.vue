<template>
    <div class="card card-border w-96 bg-base-200 relative">
        <div class="card-body">
            <input ref="descriptionRef" tabindex="1" type="text" class="input w-full" placeholder="Name" v-model="newActivity.description"> 
            <div class="flex row gap-2">
                <button tabindex="2" class="btn btn-ghost p-0 m-0" @click="newActivity.operation = newActivity.operation == Operation.credit ? Operation.debit : Operation.credit">
                    <TransactionCredit class="h-10 w-10" v-if="newActivity.operation == Operation.credit"></TransactionCredit>
                    <TransactionDebit class="h-10 w-10" v-else></TransactionDebit>
                </button>
                <MonetaryComponent tabindex="3" class="input col" v-model="formattedValue" :options="inputMonetaryOptions"/>
                <select tabindex="4" class="select" v-model="newActivity.medium">
                    <option v-for="payment of currentPaymentsList" 
                            :disabled="payment.id == '-1'" 
                            :selected="payment.id == '-1'" 
                            :value="payment"> {{ payment.name }}</option>
                </select>
            </div>
            <div class="flex row gap-2">
                <input tabindex="5" type="text" class="input w-full" placeholder="Tags" v-model="typingTag"
                    @focus="showTagSuggestions = true"
                    @blur="handleBlur"
                    @keydown.enter="testTag()"
                    @keydown.down.prevent="moveSuggestion(1)"
                    @keydown.up.prevent="moveSuggestion(-1)"
                    > 
                <ul v-if="suggestionVisible" class="menu dropdown-content  bg-base-100 rounded-box z-1 w-52 p-2 shadow-sm">
                    <li v-for="(tag, index) in suggestedTags"><a :class="['hover:btn-secondary', {'bg-base-300': index == activeSuggestion}]" @click="addTag(tag)">{{tag.name}}</a></li>
                </ul>
                <input type="date" class="input w-full" placeholder="Data" v-model="newActivity.date"> 
            </div>
            <div>
                <div v-for="tag in newActivity.tags" class="badge">{{ tag.name }}</div>
            </div>
        </div>
        <button :disabled="saveDisabled" tabindex="6" class="transition-all btn btn-circle btn-primary absolute left-1/2 -bottom-6 transform -translate-x-1/2 shadow-lg" @click="createActivity">
            <Plus class="h-6 w-6" fill="none"></Plus>
        </button>
    </div>
</template>
<script setup lang="ts">
import { ref, computed, onMounted, watch } from 'vue';
import { TransactionCredit, TransactionDebit, Plus } from '../../components/icons';
import  { type Activity, type MonetaryMedium, type Tag, Operation } from '../../models/Activity';
import ActivityService from '../../services/ActivityService'
import dayjs from 'dayjs';
import MonetaryComponent from '../../components/MonetaryComponent.vue';

const emit = defineEmits(['activity-created'])
defineProps<{
    data? : Activity
}>()

const suggestionVisible = computed(() => showTagSuggestions.value && typingTag.value !== '')
const formattedValue = computed({
    get: () => newActivity.value.value.toFixed(2),
    set: (val) => newActivity.value.value = parseFloat(val)
})
const currentPaymentsList = computed(() => {
    return [...monetaryMedia.value.filter(elem => (newActivity.value.operation == Operation.debit) || elem.isValidForCredit), {name: 'Pagamento', id: '-1', isValidForCredit: true}]
})
const suggestedTags = computed(()=> popularTags.value.filter((elem: Tag) => elem.name.includes(typingTag.value)))

const inputMonetaryOptions = {
    currency: 'BRL',
    autoDecimalDigits: true
}

const descriptionRef = ref<HTMLInputElement | null>(null);
const showTagSuggestions = ref<boolean>(false);
const popularTags = ref<Tag[]>([]);
const currentTags = ref<Tag[]>([]);
const typingTag = ref<string>('');
const saveDisabled = ref<boolean>(false);
const activeSuggestion = ref<number>(-1);
const defaultActivity = ref<Activity>({
    value : 0.0,
    medium: {name : 'Pagamento', id: '-1', isValidForCredit: true},
    description: '',
    operation: Operation.credit,
    tags: [],
    date: dayjs().format('YYYY-MM-DD')  
})
const newActivity = ref<Activity>(defaultActivity.value);
const monetaryMedia = ref<MonetaryMedium[]>([])

watch(typingTag, () => {
    if(typingTag.value !== '') activeSuggestion.value = 0
})

onMounted(async () => {
    monetaryMedia.value =  await ActivityService.getInstance().getMonetaryMedia();
    popularTags.value =  await ActivityService.getInstance().getSuggestionTags();

    console.log("popularTags.value: ", popularTags.value)
})

const handleBlur = ()=>{
    setTimeout(()=>showTagSuggestions.value = false, 200)
} 

const addTag = (tag:Tag) => {
    newActivity.value.tags?.push(tag);
    typingTag.value = '';
}

const testTag = () => {
    if(suggestedTags.value.length == 0 || activeSuggestion.value == -1){
        newActivity.value.tags?.push({id: null, name: typingTag.value})
    }else
        newActivity.value.tags?.push(popularTags.value[activeSuggestion.value])

    typingTag.value = '';    
}

const moveSuggestion = (variation: number) => {
    if(activeSuggestion.value + variation >= -1 && activeSuggestion.value + variation < suggestedTags.value.length)
        activeSuggestion.value += variation;    
}

const createActivity = async()=>{
    saveDisabled.value = true;
    const result = await ActivityService.getInstance().createActivity(newActivity.value);
    console.log("Activity create result: ", result)
    if(result){
        emit('activity-created')
        await resetForm();
        descriptionRef.value?.focus();
    }
    saveDisabled.value = false;
}

const resetForm = async() => {
    newActivity.value = defaultActivity.value;
    currentTags.value = [];

    console.log("defaultActivity.value: ", defaultActivity.value)
    console.log("newActivity.value: ", newActivity.value)

}

</script>
<style lang="sass">
.btn-circle:active
    transform: translate(-50%, 0) scale(0.97) !important
</style>
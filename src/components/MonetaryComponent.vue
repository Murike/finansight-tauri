<template>
    <input
      ref="inputRef"
      type="text"
      v-model="formattedValue"
    />
</template>

<script setup lang="ts">
import { useCurrencyInput, type CurrencyInputOptions } from 'vue-currency-input'
import { watch } from 'vue'

const props = defineProps<{
    modelValue?: string,
    options?: CurrencyInputOptions
}>()

watch(
      () => props.modelValue,
      (value) => {
        if(value) setValue(parseFloat(value));
      }
    );

defineOptions({
    name: 'CurrencyInput'
})

const { inputRef, setValue, formattedValue } = useCurrencyInput(props.options || { currency: 'BRL' })
  </script>
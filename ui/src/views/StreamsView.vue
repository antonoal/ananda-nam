<template>
  <Message severity="error" v-if="error">{{ error }}</Message>
  <div class="container">
    <Card>
      <template #title>{{ $t('views.streams') }}</template>
      <template #content>
        <Button
          outlined
          :label="$t('menu.new')"
          icon="pi pi-plus"
          severity="info"
          class="mr-2"
          @click="openNewDialog"
        />
        <Divider />
        <Menu ref="rowMenu" :model="rowMenuItems" popup />
        <DataTable
          v-model:contextMenuSelection="selected"
          removableSort
          stripedRows
          dataKey="id"
          class="w-full"
          :value="streams"
        >
          <Column field="name" :header="$t('streams.columns.name')" sortable></Column>
          <Column field="start_year" header="Start Year" sortable></Column>
          <Column :exportable="false">
            <template #body="slotProps">
              <div class="flex justify-end">
                <Button
                  icon="pi pi-ellipsis-h"
                  text
                  severity="secondary"
                  @click="showRowMenu($event, slotProps.data)"
                />
              </div>
            </template>
          </Column>
        </DataTable>
      </template>
    </Card>
    <Dialog
      v-model:visible="newDialog"
      :style="{ width: '450px' }"
      :header="t(`dialog.${selected ? 'edit' : 'new'}`, { s: t('streams.stream') })"
      :modal="true"
      class="p-fluid"
    >
      <form>
        <div class="flex flex-col gap-2 text-gray-700 dark:text-white">
          <label for="newName" class="block">{{ t('streams.columns.name') }}:</label>
          <InputText
            v-model="name"
            v-bind="nameAttrs"
            id="name"
            autofocus
            :class="[{ 'border-red-500': errors.name }]"
          />
          <small class="text-red-500 dark:text-red-800">{{ errors.name }}</small>
        </div>
        <div class="flex flex-col gap-2 text-gray-700 dark:text-white">
          <label for="newName" class="block">Start Year:</label>
          <InputNumber
            v-model="startYear"
            v-bind="startYearAttrs"
            :useGrouping="false"
            :min="1970"
            :max="3000"
            id="startYear"
            :class="[{ 'border-red-500': errors.startYear }]"
          />
          <small class="text-red-500 dark:text-red-800">{{ errors.startYear }}</small>
        </div>
        <div class="flex justify-end">
          <Button
            type="reset"
            :label="t('menu.cancel')"
            icon="pi pi-times"
            text
            @click="closeNewDialog"
          />
          <Button type="submit" :label="t('menu.save')" icon="pi pi-check" text @click="upsert" />
        </div>
      </form>
    </Dialog>
    <Dialog
      v-model:visible="deleteStreamDialog"
      :style="{ width: '450px' }"
      :header="t('dialog.confirm')"
      :modal="true"
    >
      <div class="confirmation-content">
        <i class="pi pi-exclamation-triangle mr-3 text-3xl" />
        <span v-if="selected">{{ t('dialog.confirmDelete', { s: selected.name }) }}</span>
      </div>
      <template #footer>
        <Button
          type="reset"
          :label="t('menu.no')"
          icon="pi pi-times"
          text
          @click="deleteStreamDialog = false"
        />
        <Button
          type="submit"
          :label="t('menu.yes')"
          icon="pi pi-check"
          text
          @click="deleteSelected"
        />
      </template>
    </Dialog>
  </div>
</template>

<script setup lang="ts">
////////////////
// TODO: error handling with i18n
//   - dup key vialation
//   - network error
//   - permissions error

import type Stream from '@/models/Stream'
import { useStreamsStore } from '@/store/streams'
import Column from 'primevue/column'
import DataTable from 'primevue/datatable'
import Card from 'primevue/card'
import { onMounted } from 'vue'
import { ref } from 'vue'
import Button from 'primevue/button'
import Menu from 'primevue/menu'
import Divider from 'primevue/divider'
import Dialog from 'primevue/dialog'
import { useI18n } from 'vue-i18n'
import Message from 'primevue/message'
import InputText from 'primevue/inputtext'
import InputNumber from 'primevue/inputnumber'
import * as yup from 'yup'
import { useForm } from 'vee-validate'
import { useRoute } from 'vue-router'
import { onBeforeRouteUpdate } from 'vue-router'

const selected = ref<Stream | null>(null)
const rowMenu = ref()
const deleteStreamDialog = ref(false)
const newDialog = ref(false)
const store = useStreamsStore()
const streams = ref<Stream[]>([])
const error = ref('')
const { t } = useI18n()
const schoolId = ref(-1)
const route = useRoute()

const schema = yup.object().shape({
  name: yup
    .string()
    .trim()
    .min(3, 'Name must be at least 3 characters long')
    .required('Name is required'),
  startYear: yup.number().min(1970).max(3000).required()
})

const { defineField, resetForm, errors, handleSubmit, setValues } = useForm({
  validationSchema: schema
})

const [name, nameAttrs] = defineField('name', { validateOnModelUpdate: false })
const [startYear, startYearAttrs] = defineField('startYear', { validateOnModelUpdate: false })

const rowMenuItems = ref([
  {
    label: t('menu.edit'),
    icon: 'pi pi-fw pi-file-edit',
    command: () => openEditDialog()
  },
  { label: t('menu.delete'), icon: 'pi pi-fw pi-times', command: () => confirmDelete() }
])

const confirmDelete = () => {
  deleteStreamDialog.value = true
}

const showRowMenu = (event: Event, data: Stream) => {
  selected.value = data
  rowMenu.value.show(event)
}

const closeNewDialog = () => {
  newDialog.value = false
}

const openEditDialog = () => {
  resetForm()
  setValues({ name: selected.value?.name, startYear: selected.value?.start_year })
  newDialog.value = true
}

const openNewDialog = () => {
  resetForm()
  selected.value = null
  newDialog.value = true
}

const deleteSelected = async () => {
  error.value = ''
  deleteStreamDialog.value = false
  try {
    if (selected.value) {
      await store.deleteStream(schoolId.value, selected.value.id)
      await store.fetchStreams(schoolId.value)
      streams.value = store.streams
    }
  } catch (e: any) {
    error.value = e.message || t('loginError') //FIXME:
  }
}

const upsert = handleSubmit(async () => {
  error.value = ''
  const newStream = {
    name: name.value,
    start_year: startYear.value
  }
  try {
    if (selected.value) {
      await store.updateStream(schoolId.value, selected.value.id, newStream)
    } else {
      await store.addStream(schoolId.value, newStream)
    }
    await store.fetchStreams(schoolId.value)
    streams.value = store.streams
    newDialog.value = false
    resetForm()
  } catch (e: any) {
    error.value = e.message || t('loginError') //FIXME:
  }
})

const load = async (schoolIdOpt: string | string[]) => {
  if (typeof schoolIdOpt === 'string') {
    schoolId.value = parseInt(schoolIdOpt, 10)
    await store.fetchStreams(schoolId.value)
    streams.value = store.streams
  }
}

onBeforeRouteUpdate(async (to) => {
  await load(to.params.schoolId)
})

onMounted(async () => {
  await load(route.params.schoolId)
})
</script>

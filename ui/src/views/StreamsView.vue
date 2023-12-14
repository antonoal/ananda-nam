<template>
  <Message severity="error" v-if="error">{{ error }}</Message>
  <div class="container">
    <Card>
      <template #title>{{ $t('views.streams') }}</template>
      <template #content>
        <Button
          size="small"
          outlined
          label="New"
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
          size="small"
          class="w-full"
          :value="streams"
        >
          <Column field="name" :header="$t('streams.columns.name')" sortable></Column>
          <Column :exportable="false">
            <template #body="slotProps">
              <div class="flex justify-end">
                <Button
                  size="small"
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
      :header="selected ? 'Edit Stream' : 'New Stream'"
      :modal="true"
      class="p-fluid"
    >
      <form>
        <div class="flex flex-col gap-2 text-gray-700 dark:text-white">
          <label for="newName" class="block text-sm font-medium">Name:</label>
          <InputText
            v-model="name"
            v-bind="nameAttrs"
            id="name"
            size="small"
            autofocus
            :class="[{ 'border-red-500': errors.name }]"
          />
          <small class="text-red-500 dark:text-red-800">{{ errors.name }}</small>
        </div>
        <div class="flex justify-end">
          <Button type="reset" label="Cancel" icon="pi pi-times" text @click="closeNewDialog" />
          <Button type="submit" label="Save" icon="pi pi-check" text @click="upsert" />
        </div>
      </form>
    </Dialog>
    <Dialog
      v-model:visible="deleteStreamDialog"
      :style="{ width: '450px' }"
      header="Confirm"
      :modal="true"
    >
      <div class="confirmation-content">
        <i class="pi pi-exclamation-triangle mr-3 text-3xl" />
        <span v-if="selected">Are you sure you want to delete the selected products?</span>
      </div>
      <template #footer>
        <Button
          type="reset"
          label="No"
          icon="pi pi-times"
          text
          @click="deleteStreamDialog = false"
        />
        <Button type="submit" label="Yes" icon="pi pi-check" text @click="deleteSelected" />
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
import { streamsStore } from '@/store/streams'
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
import * as yup from 'yup'
import { useForm } from 'vee-validate'

const selected = ref<Stream | null>(null)
const rowMenu = ref()
const deleteStreamDialog = ref(false)
const newDialog = ref(false)
const store = streamsStore()
const streams = ref<Stream[]>([])
const error = ref('')
const { t } = useI18n()

const schema = yup.object().shape({
  name: yup
    .string()
    .trim()
    .min(3, 'Name must be at least 3 characters long')
    .required('Name is required')
})

const { defineField, resetForm, errors, handleSubmit, setValues } = useForm({
  validationSchema: schema
})

const [name, nameAttrs] = defineField('name', { validateOnModelUpdate: false })

const rowMenuItems = ref([
  {
    label: 'Edit',
    icon: 'pi pi-fw pi-file-edit',
    command: () => openEditDialog()
  },
  { label: 'Delete', icon: 'pi pi-fw pi-times', command: () => confirmDelete() }
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
  setValues({ name: selected.value?.name })
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
      await store.deleteStream(selected.value.id)
      await store.fetchStreams()
      streams.value = store.streams
    }
  } catch (e: any) {
    error.value = e.message || t('loginError') //FIXME:
  }
}

const upsert = handleSubmit(async () => {
  error.value = ''
  const newStream = {
    name: name.value
  }
  try {
    if (selected.value) {
      await store.updateStream(selected.value.id, newStream)
    } else {
      await store.addStream(newStream)
    }
    await store.fetchStreams()
    streams.value = store.streams
    newDialog.value = false
    resetForm()
  } catch (e: any) {
    error.value = e.message || t('loginError') //FIXME:
  }
})

onMounted(async () => {
  await store.fetchStreams()
  streams.value = store.streams
})
</script>

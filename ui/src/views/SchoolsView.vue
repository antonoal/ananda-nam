<template>
  <Message severity="error" v-if="error">{{ error }}</Message>
  <div class="container">
    <Card>
      <template #title>{{ $t('views.schools') }}</template>
      <template #content>
        <Button
          size="small"
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
          size="small"
          class="w-full"
          :value="schools"
        >
          <Column field="name" :header="$t('schools.columns.name')" sortable></Column>
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
      :header="t(`dialog.${selected ? 'edit' : 'new'}`, { s: t('schools.school') })"
      :modal="true"
      class="p-fluid"
    >
      <form>
        <div class="flex flex-col gap-2 text-gray-700 dark:text-white">
          <label for="newName" class="block text-sm font-medium"
            >{{ t('schools.columns.name') }}:</label
          >
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
      v-model:visible="deleteSchoolDialog"
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
          @click="deleteSchoolDialog = false"
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

import type School from '@/models/School'
import { schoolsStore } from '@/store/schools'
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

const selected = ref<School | null>(null)
const rowMenu = ref()
const deleteSchoolDialog = ref(false)
const newDialog = ref(false)
const store = schoolsStore()
const schools = ref<School[]>([])
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
    label: t('menu.edit'),
    icon: 'pi pi-fw pi-file-edit',
    command: () => openEditDialog()
  },
  { label: t('menu.delete'), icon: 'pi pi-fw pi-times', command: () => confirmDelete() }
])

const confirmDelete = () => {
  deleteSchoolDialog.value = true
}

const showRowMenu = (event: Event, data: School) => {
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
  deleteSchoolDialog.value = false
  try {
    if (selected.value) {
      await store.deleteSchool(selected.value.id)
      await store.fetchSchools()
      schools.value = store.schools
    }
  } catch (e: any) {
    error.value = e.message || t('loginError') //FIXME:
  }
}

const upsert = handleSubmit(async () => {
  error.value = ''
  const newSchool = {
    name: name.value
  }
  try {
    if (selected.value) {
      await store.updateSchool(selected.value.id, newSchool)
    } else {
      await store.addSchool(newSchool)
    }
    await store.fetchSchools()
    schools.value = store.schools
    newDialog.value = false
    resetForm()
  } catch (e: any) {
    error.value = e.message || t('loginError') //FIXME:
  }
})

onMounted(async () => {
  await store.fetchSchools()
  schools.value = store.schools
})
</script>

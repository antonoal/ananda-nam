<template>
  <Message severity="error" v-if="error">{{ error }}</Message>
  <div class="container">
    <Card>
      <template #title>{{ $t('views.groups') }}</template>
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
        <Menu ref="leafRowMenu" :model="leafRowMenuItems" popup />
        <TreeTable
          removableSort
          stripedRows
          resizableColumns
          :value="groups"
          dataKey="id"
          class="w-full"
          size="small"
          v-model:contextMenuSelection="selected"
        >
          <Column field="name" :header="$t('groups.columns.name')" expander></Column>
          <Column field="responsible" header="Responsible"></Column>
          <Column field="student" header="Student"></Column>
          <Column :exportable="false">
            <template #body="slotProps">
              <div class="flex justify-end">
                <Button
                  icon="pi pi-ellipsis-h"
                  text
                  severity="secondary"
                  @click="showRowMenu($event, slotProps.node)"
                />
              </div>
            </template>
          </Column>
        </TreeTable>
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
          <label for="newName" class="block text-sm font-medium">{{
            t('streams.columns.name')
          }}</label>
          <InputText
            v-model="name"
            v-bind="nameAttrs"
            id="name"
            autofocus
            autocomplete="false"
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
      v-model:visible="deleteStreamDialog"
      :style="{ width: '450px' }"
      :header="t('dialog.confirm')"
      :modal="true"
    >
      <div class="confirmation-content">
        <i class="pi pi-exclamation-triangle mr-3 text-3xl" />
        <span v-if="selected">{{ t('dialog.confirmDelete', { s: selected.data.name }) }}</span>
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

import { groupsStore } from '@/store/groups'
import Column from 'primevue/column'
import Card from 'primevue/card'
import { onMounted, ref, computed } from 'vue'
import Button from 'primevue/button'
import Menu from 'primevue/menu'
import Divider from 'primevue/divider'
import Dialog from 'primevue/dialog'
import { useI18n } from 'vue-i18n'
import Message from 'primevue/message'
import InputText from 'primevue/inputtext'
import * as yup from 'yup'
import { useForm } from 'vee-validate'
import { type Group } from '@/models/Group'
import TreeTable from 'primevue/treetable'
import { onBeforeRouteUpdate } from 'vue-router'
import { useRoute } from 'vue-router'

const selected = ref<Group | null>(null)
const rowMenu = ref()
const leafRowMenu = ref()
const deleteStreamDialog = ref(false)
const newDialog = ref(false)
const store = groupsStore()
const groups = ref<Group[]>([])
const error = ref('')
const streamId = ref(-1)
const schoolId = ref(-1)
const route = useRoute()
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
  { label: 'Add Child', command: () => openNewDialog() },
  { label: 'Appoint Responsible' },
  { label: t('menu.delete'), icon: 'pi pi-fw pi-times', command: () => confirmDelete() }
])

const leafRowMenuItems = ref([
  {
    label: 'Edit Leaf',
    icon: 'pi pi-fw pi-file-edit'
    // command: () => openEditDialog()
  },
  { label: t('menu.delete'), icon: 'pi pi-fw pi-times', command: () => confirmDelete() }
])

const confirmDelete = () => {
  deleteStreamDialog.value = true
}

const showRowMenu = (event: Event, data: Group) => {
  selected.value = data
  // if (isLeafSelected.value) {
  //   leafRowMenu.value.show(event)
  // } else {
  rowMenu.value.show(event)
  // }
}

const closeNewDialog = () => {
  newDialog.value = false
}

const openEditDialog = () => {
  resetForm()

  setValues({ name: selected.value?.data.name })
  newDialog.value = true
}

const openNewDialog = () => {
  resetForm()
  newDialog.value = true
}

const deleteSelected = async () => {
  error.value = ''
  deleteStreamDialog.value = false
  try {
    if (selected.value) {
      await store.deleteGroup(schoolId.value, selected.value.key)
      await refresh()
    }
  } catch (e: any) {
    error.value = e.message || t('loginError') //FIXME:
  }
}

const upsert = handleSubmit(async () => {
  error.value = ''

  try {
    await store.addGroup(schoolId.value, name.value, streamId.value, selected.value)
    // if (selected.value) {
    //   await store.updateStream(selected.value.id, newStream)
    // } else {
    //   await store.addStream(newStream)
    // }
    await refresh()
    newDialog.value = false
    resetForm()
  } catch (e: any) {
    error.value = e.message || t('loginError') //FIXME:
  }
  selected.value = null
})

const load = async (schoolIdOpt: string | string[], streamIdOpt: string | string[]) => {
  if (typeof streamIdOpt === 'string' && typeof schoolIdOpt === 'string') {
    streamId.value = parseInt(streamIdOpt, 10)
    schoolId.value = parseInt(schoolIdOpt, 10)

    refresh()
  }
}

const refresh = async () => {
  await store.fetchGroups(schoolId.value)
  groups.value = store.getGroup(streamId.value)
}

onMounted(async () => {
  await load(route.params.schoolId, route.params.streamId)
})

onBeforeRouteUpdate(async (to) => {
  await load(to.params.schoolId, to.params.streamId)
})

onMounted(async () => {})
</script>

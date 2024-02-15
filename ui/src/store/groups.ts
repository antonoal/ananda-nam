import { defineStore } from 'pinia'
import axiosClient from '@/axiosClient'
import { type DbGroup, type Group, type GroupData } from '@/models/Group'

export const groupsStore = defineStore('groups', {
  state: () => ({
    lastId: 0,
    groups: [] as Group[]
  }),
  actions: {
    async fetchGroups(schoolId: number) {
      try {
        const response = await axiosClient.get<GroupsResponse>(`/schools/${schoolId}/groups`)
        this.groups = fromDbGroups(response.data.groups)
      } catch (error) {
        console.error('Error fetching groups:', error)
        throw error
      }
    },
    getGroup(streamId: number) {
      return this.groups.filter((g) => g.data.stream_id === streamId)
    },
    async addGroup(schoolId: number, groupName: string, stream_id: number, parent: Group | null) {
      const parentId = parent ? +parent.key : null

      const newGroup: NewGroup = {
        name: groupName,
        parent_id: parentId,
        year_id: parent?.data.year_id ?? null,
        stream_id: stream_id
      }

      try {
        await axiosClient.post(`/schools/${schoolId}/groups`, { group: newGroup })
      } catch (error) {
        console.error('Error adding group:', error)
        throw error
      }
    },
    async deleteGroup(schoolId: number, id: String) {
      try {
        await axiosClient.delete(`/schools/${schoolId}/groups/${id}`)
      } catch (error) {
        console.error('Error deleting group:', error)
        throw error
      }
    }
    // async updateSchool(id: number, newSchool: NewSchool) {
    //   try {
    //     await axiosClient.put(`/schools/${id}`, { school: newSchool })
    //   } catch (error) {
    //     console.error('Error updating school:', error)
    //     throw error
    //   }
    // }
  }
})

function fromDbGroups(dbGroups: DbGroup[]): Group[] {
  // Create a map to store Group objects by their id
  const groupsMap: Map<number, Group> = new Map()

  // Populate the map with Group objects with empty children arrays
  dbGroups.forEach((dbGroup) => {
    groupsMap.set(dbGroup.id, {
      key: dbGroup.id.toString(),
      data: {
        year_id: dbGroup.year_id,
        stream_id: dbGroup.stream_id,
        name: dbGroup.name
      },
      children: []
    })
  })

  // Build the parent-child relationships
  dbGroups.forEach((dbGroup) => {
    const parentGroup = dbGroup.parent_id !== null ? groupsMap.get(dbGroup.parent_id) : null
    const childGroup = groupsMap.get(dbGroup.id)
    if (parentGroup && childGroup) {
      parentGroup.children.push(childGroup)
    }
  })

  // Find and return the top-level groups (those with no parent)
  const topLevelGroups: Group[] = []
  dbGroups.forEach((g) => {
    const group = groupsMap.get(g.id)
    if (g.parent_id === null && group) {
      topLevelGroups.push(group)
    }
  })

  return topLevelGroups
}

interface GroupsResponse {
  groups: Array<DbGroup>
}

export interface NewGroup extends Omit<DbGroup, 'id'> {}

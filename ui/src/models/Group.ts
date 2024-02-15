interface DbGroup {
  id: number
  parent_id: number | null
  year_id: number | null
  stream_id: number
  name: string
}

interface Group {
  key: string
  data: GroupData
  children: Array<Group>
}

interface GroupData {
  year_id: number | null
  stream_id: number
  name: string
}

export type { DbGroup, Group, GroupData }

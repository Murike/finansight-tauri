import { Activity, Tag } from "./Activity"

export type StaticFilter = {
    id: string,
    initialDate: string,
    finalDate: string,
	tags: Tag[],
    Activities: Activity[]
}

export type ActivityFilter = {
    initialDate?: string,
    finalDate?: string,
    tags?: Tag[],
}
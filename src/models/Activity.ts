export enum Operation {
    credit = "credit",
	debit = "debit",
}


export type Activity = {
    id?: string,
    value : number,
    medium: MonetaryMedium,
    operation: Operation,
    description: string,
    date: string,
    tags?: Tag[]
}


export type MonetaryMedium = {
    name: string, 
    id: string, 
    isValidForCredit: boolean
}

export type Tag = {
	id: string | null, 
	name: string
}

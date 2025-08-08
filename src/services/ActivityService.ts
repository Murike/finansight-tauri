import type { Activity, MonetaryMedium, Tag } from "../models/Activity";
import type { DateRange } from "../models/ActivityDate";
import { invoke } from "@tauri-apps/api/core";
import { ActivityFilter, StaticFilter } from "../models/Filters";

export default class ActivityService {
    private static instance: ActivityService = new ActivityService();
    
    constructor(){
    }

    static getInstance(){
        return this.instance;
    }

    async createActivity(activity: Activity) : Promise<string> {
        return await invoke<string>('add_activity', { activity });
    }

    async getActivities(monthValue: DateRange, filters?: ActivityFilter) : Promise<Activity[]> {
        console.log("monthValue: ", monthValue)
        
        const activities = await invoke<Activity[]>('get_activities', {filters});
        console.log("activities: ", activities);
        return activities;
    }

    async deleteActivity(activityId: string) : Promise<number> {
        try{
            return await invoke<number>('delete_activity', {activityId});
        }catch(e: any){
            console.log("Current ERROR: ", e)
            throw new Error(e.msg);
        }
    }

    async getMonetaryMedia() : Promise<MonetaryMedium[]>{
        return await invoke<MonetaryMedium[]>('get_monetary_media');
    }

    async getSuggestionTags() : Promise<Tag[]>{
        return await invoke<Tag[]>('get_suggestion_tags');
    }

    async getStaticFilters() : Promise<StaticFilter[]>{
        return await invoke<StaticFilter[]>('get_static_filters');
    }

}
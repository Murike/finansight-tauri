import type { Activity, MonetaryMedium, Tag } from "../models/Activity";
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

    async getActivities(filters?: ActivityFilter) : Promise<Activity[]> {
        const activities = await invoke<Activity[]>('get_activities', {filters});
        console.log("activities: ", activities);
        return activities;
    }

    async addFilterTag(filterId : string, tag: Tag) : Promise<StaticFilter> {
        const tagId = tag.id;
        return await invoke<StaticFilter>('add_filter_tag', {filterId, tagId});
    }

    async removeFilterTag(filterId : string, tag: Tag) : Promise<StaticFilter> {
        const tagId = tag.id;
        return await invoke<StaticFilter>('remove_filter_tag', {filterId, tagId});
    }

    async updateFilterDaterange(filter : StaticFilter) : Promise<StaticFilter> {
        return await invoke<StaticFilter>('update_filter_daterange', {filter});
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
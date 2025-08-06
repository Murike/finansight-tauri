import dayjs from "dayjs"
import localeData from 'dayjs/plugin/localeData';
import utc from 'dayjs/plugin/utc';
import type { ActivityDate, DateRange } from '../models/ActivityDate'

// Initialize dayjs plugins immediately
dayjs.extend(localeData);
dayjs.extend(utc);

export class utils{
    static months: dayjs.MonthNames = dayjs.months();

    static getMonthsList(){
        return utils.months?.reduce((agg: ActivityDate[], month: string, index: number) => {
            agg.push({label: month, value: index});
            return agg;
        }, [])
    }

    static getCurrentMonth(){
        const month = dayjs().utc().month();
        const newDate: ActivityDate = {label: utils.months?.[month], value: month}
        return newDate
    }

    static getMonthInitialFinal(monthIndex? : number, year?: number){
        const usedMonthIndex = monthIndex ?? dayjs().month();
        const usedYear = year ?? dayjs().year();

        const lastDay = dayjs().month(usedMonthIndex).endOf('month').date()
        const rangeDate: DateRange = {
            initialDate: dayjs(`${usedYear}-${usedMonthIndex + 1}-01`).utc().startOf('day').toISOString()  , 
            finalDate: dayjs(`${usedYear}-${usedMonthIndex + 1}-${lastDay}`).utc().endOf('day').toISOString()  
        };

        return rangeDate;
    }
}
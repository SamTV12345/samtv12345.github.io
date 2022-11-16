import {FC} from "react";
import {BadgeProp} from "./BetaBadge";

export const ErrorBadge:FC<BadgeProp> = ({classname})=>{
    return <span
        className={`bg-red-100 text-red-800 text-xs font-semibold mr-2 px-2.5 py-0.5 rounded dark:bg-red-200 dark:text-red-900 ${classname}`}>Error</span>
}

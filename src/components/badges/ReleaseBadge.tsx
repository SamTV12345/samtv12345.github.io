import {BadgeProp} from "./BetaBadge";
import {FC} from "react";

export const ReleaseBadge:FC<BadgeProp> = ({classname})=>{
    return <span
        className={`bg-green-100 text-green-800 text-xs font-semibold mr-2 px-2.5 py-0.5 rounded dark:bg-green-200 dark:text-green-900 ${classname}`}>Release</span>

}

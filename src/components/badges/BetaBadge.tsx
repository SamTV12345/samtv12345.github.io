import {FC} from "react";

export interface BadgeProp {
    classname:string
}


export const BetaBadge:FC<BadgeProp> = ({classname})=>{
    return <span
        className={`bg-yellow-100 text-yellow-800 text-xs font-semibold mr-2 px-2.5 py-0.5 rounded dark:bg-yellow-200 dark:text-yellow-900 ${classname}`}>Beta</span>

}

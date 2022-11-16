import {FC} from "react";
import {ReleaseBadge} from "./ReleaseBadge";
import {AlphaBadge} from "./AlphaBadge";
import {BetaBadge} from "./BetaBadge";
import {ErrorBadge} from "./ErrorBadge";

interface BadgeSelector{
    type:"alpha"| "release"|"beta"|"error",
    classname:string
}


export const BadgeSelector:FC<BadgeSelector> = ({type,classname})=>{

    const selectBadge = (type:"alpha"| "release"|"beta"|"error")=>{
        switch (type){
            case "release": return <ReleaseBadge classname={classname}/>
            case "alpha": return <AlphaBadge classname={classname}/>
            case "beta": return <BetaBadge classname={classname}/>
            case "error":return <ErrorBadge classname={classname}/>
        }
    }

    return <span>{selectBadge(type)}</span>
}

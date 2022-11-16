import {FC} from "react";
import '../App.css'

interface SkillProps {
    title:string,
    children?:any
}

export const Skill:FC<SkillProps> =({children,title})=>{
    return <div className="bg-white text-black p-2">
        <h1 className="text-center text-black">{title}</h1>
        {children}
    </div>
}

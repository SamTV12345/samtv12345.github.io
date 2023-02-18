import React, {FC} from "react";
import {scrollToElement} from "./Title";

interface ContainerProps {
    children:any,
    classNames: string,
    id: string
}


export const Container:FC<ContainerProps> = ({children, classNames, id})=>{
    return <div className="flex h-screen relative" style={{zIndex:100}} id={id}>
        <div className={classNames}>
            {children}
        </div>
    </div>
}

//"bg-gray-200 m-auto h-2/4 w-3/4 p-6 rounded-2xl grid grid-cols-3 gap-5 overflow-hidden"

import {FC} from "react";

interface ContainerProps {
    children:any,
    classNames: string
}


export const Container:FC<ContainerProps> = ({children, classNames})=>{
    return <div className="flex h-screen relative" style={{zIndex:100}}>
        <div className={classNames}>
            {children}
        </div>
    </div>
}

//"bg-gray-200 m-auto h-2/4 w-3/4 p-6 rounded-2xl grid grid-cols-3 gap-5 overflow-hidden"

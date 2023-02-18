import {BlendInComponent} from "./BlendInComponent";
import React from "react";

export const scrollToElement = (id:string) =>{
    const element = document.getElementById(id);
    element?.scrollIntoView({behavior: "smooth"});
}

export const Title = ()=>{
    return <BlendInComponent id="title">
        <div className="w-full h-screen grid place-items-center">
            <h1 className="text-4xl m-auto p-6 text-white bg-neutral-600 rounded-full">
                Welcome to the homepage of SamTV
            </h1>
            <div  onClick={()=>scrollToElement('skills')} className="absolute bottom-1.5 animate-[bounce_3s_ease-in-out_infinite] bg-white dark:bg-slate-800 p-2 w-10 h-10 ring-1 ring-slate-900/5 dark:ring-slate-200/20 shadow-lg rounded-full flex items-center justify-center">
                <svg className="w-6 h-6 text-violet-500" fill="none" strokeLinecap="round" strokeLinejoin="round"
                     strokeWidth="2" viewBox="0 0 24 24" stroke="currentColor">
                    <path d="M19 14l-7 7m0 0l-7-7m7 7V3"></path>
                </svg>
            </div>
        </div>
    </BlendInComponent>
}

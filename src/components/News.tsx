import {Container} from "./Container";
import '../App.css'
import {useState} from "react";
import news from '../assets/news.json'
import {BadgeSelector} from "./badges/BadgeSelector";
export const News = ()=>{
    const [id, setIdOfNews] = useState<number>(0)

    return <Container
        classNames="bg-gray-200 m-auto h-3/4 p-6 rounded-2xl grid grid-cols-2 gap-5  overflow-auto w-full md:w-3/4">
        <div className="h-full grid grid-cols-1 auto-rows-min gap-4">
            <h2 className="text-center text-xl">Latest news</h2>
            {news.map(n=><button className={`${id===n.id?'bg-gray-300':'bg-gray-100'} rounded text-left pl-1 p-2`} onClick={()=>setIdOfNews(n.id)}>{n.name}<BadgeSelector type={n.badge as any} classname={"ml-2"}/></button>)}

        </div>
        <div className="h-full overflow-auto">
            <h2 className="text-center text-xl">Description</h2>
            <div className="m-4 p-2 bg-gray-100 rounded">{news.filter(n=>n.id===id).map(selectedNews=><div>{selectedNews.description}</div>)}</div>
        </div>
    </Container>
}

import {Container} from "./Container";
import React from "react";
import projects from "../assets/projects.json"
import {Fade} from "react-awesome-reveal";

export const Projects = () => {
    return <Container
        classNames="bg-gray-200 m-auto h-3/4 p-6 rounded-2xl grid grid-cols-3 gap-5  overflow-auto w-full md:w-3/4">
        <div className="text-center text-2xl col-span-3">Projects</div>
        {
            projects.map(f => <Fade className="col-span-3 p-2 rounded bg-gray-100" key={f.link} duration={1200} triggerOnce>
                    <a href={f.link+"link"} key={f.link} target="_blank">
                        <h2 className="text-lg">{f.name}</h2>
                        <p className="pt-4 text-sm">{f.description}</p>
                    </a>
                </Fade>
            )
        }
    </Container>
}

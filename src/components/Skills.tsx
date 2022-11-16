import {Fade} from "react-awesome-reveal";
import {Skill} from "./Skill";
import React from "react";
import {Container} from "./Container";

export const Skills = ()=>{
    return <Container classNames="bg-gray-200 m-auto h-11/12 w-full md:w-3/4 p-6 rounded-2xl grid md:grid-cols-3 grid-cols-2 gap-5 overflow-hidden">
            <div className="text-center text-2xl md:col-span-3 col-span-2">Skills</div>
            <Fade direction={"left"} triggerOnce>
                <Skill title="K8s"/>
            </Fade>
            <Fade direction={"down"} delay={400} triggerOnce>
                <Skill title="Docker"/>
            </Fade>
            <Fade direction={"right"} delay={500} triggerOnce>
                <Skill title="Java"/>
            </Fade>
            <Fade delay={600} direction={"left"} triggerOnce>
                <Skill title="JavaScript"/>
            </Fade>
            <Fade delay={700} direction={"down"} triggerOnce>
                <Skill title="Typescript"/>
            </Fade>
            <Fade delay={800} direction={"right"} triggerOnce>
                <Skill title="React"/>
            </Fade>
            <Fade delay={800} direction={"left"} triggerOnce>
                <Skill title="Python"/>
            </Fade>
            <Fade delay={800} direction={"up"} triggerOnce>
                <Skill title="Spring"/>
            </Fade>
            <Fade delay={800} direction={"right"} triggerOnce>
                <Skill title="Quarkus"/>
            </Fade>
            <Fade delay={800} direction={"left"} triggerOnce>
                <Skill title="Keycloak"/>
            </Fade>
            <Fade delay={900} direction={"up"} triggerOnce>
                <Skill title={"Pfsense"}/>
            </Fade>
            <Fade direction={"right"} delay={900} triggerOnce>
                <Skill title={"Proxmox"}/>
            </Fade>
    </Container>
}

import React from 'react'
import ReactDOM from 'react-dom/client'
import App from './App'
import './index.css'
import "animate.css/animate.min.css";


const Root = () => {
    const particlesInit = useCallback(async (engine: Engine) => {
        console.log(engine);

        // you can initialize the tsParticles instance (engine) here, adding custom shapes or presets
        // this loads the tsparticles package bundle, it's the easiest method for getting everything ready
        // starting from v2 you can add only the features you need reducing the bundle size
        await loadFull(engine);
    }, []);

    const particlesLoaded = useCallback(async () => {
    }, []);
    // @ts-ignore
    return (
        <div>
            <Particles
                id="tsparticles"
                init={particlesInit}
                loaded={particlesLoaded}
                options={{
                    background: {
                        color: {
                            value: "#000",
                        },
                    },
                    fpsLimit: 120,
                    interactivity: {
                        events: {
                            onClick: {
                                enable: true,
                                mode: "push",
                            },
                            onHover: {
                                enable: true,
                                mode: "repulse",
                            },
                            resize: true,
                        },
                        modes: {
                            push: {
                                quantity: 4
                            },
                            repulse: {
                                distance: 200,
                                duration: 0.4,
                            },
                        },
                    },
                    particles: {
                        color: {
                            value: "#ffffff",
                        },
                        links: {
                            color: "#ffffff",
                            distance: 150,
                            enable: true,
                            opacity: 0.5,
                            width: 1,
                        },
                        collisions: {
                            enable: true,
                        },
                        move: {
                            direction: "none",
                            enable: true,
                            outModes: {
                                default: "out",
                            },
                            attract: {
                                enable: false,
                                rotateX: 600,
                                rotateY:1200
                            },
                            random: false,
                            speed: 6,
                            straight: false,
                        },
                        number: {
                            density: {
                                enable: true,
                                area: 800
                            },
                            max:50,
                            value: 80,
                        },
                        opacity: {
                            value: 0.5,
                            random:false,
                            anim: {
                                enable:false,
                                speed:1,
                                opacity_min:0.1,
                                sync:false
                            }
                        },
                        shape: {
                            type: "circle",
                            stroke:{
                                width: 0,
                                color: '#000'
                            },
                            polygon: {
                                nb_sides:5
                            }
                        },
                        size: {
                            value: 5,
                            random:true,
                            anim: {
                                enable: false,
                                speed:40,
                                size_min:0.1,
                                sync:false
                            }
                        },
                    },
                    detectRetina: true,
                }}
           />
        </div>
    );
};

ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
  <React.StrictMode>
      <Root/>
      <App />
  </React.StrictMode>
)

import { useCallback } from "react";
import type { Container, Engine } from "tsparticles-engine";
import Particles from "react-tsparticles";
import { loadFull } from "tsparticles";

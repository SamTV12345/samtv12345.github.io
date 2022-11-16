import './App.css'
import React  from 'react';
import {Title} from "./components/Title";
import {Skills} from "./components/Skills";
import {Projects} from "./components/Projects";
import {Footer} from "./components/Footer";

const App = () => {

    return <React.Fragment>
        <Title/>
        <Skills/>
        <Projects/>
        <Footer/>
    </React.Fragment>
}

export default App

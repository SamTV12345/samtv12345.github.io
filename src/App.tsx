import './App.css'
import React  from 'react';
import {Title} from "./components/Title";
import {Skills} from "./components/Skills";
import {Projects} from "./components/Projects";
import {Footer} from "./components/Footer";
import {News} from "./components/News";
import {PageUpSymbol} from "./components/PageUpSymbol";

const App = () => {

    return <React.Fragment>
        <Title/>
        <Skills/>
        <Projects/>
        <News/>
        <Footer/>
        <PageUpSymbol/>
    </React.Fragment>
}

export default App

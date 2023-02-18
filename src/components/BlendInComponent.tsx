import { keyframes } from "@emotion/react";
import { Reveal } from "react-awesome-reveal";
import {FC} from "react";

const customAnimation = keyframes`
  from {
    opacity: 0;
  }
  to {
    opacity: 1;
  }
`;

interface BlendInComponentProps {
    children:any,
    id: string
}

export const BlendInComponent:FC<BlendInComponentProps> = ({children})=> {
    return <Reveal keyframes={customAnimation}  duration={10000} className="flex h-screen" triggerOnce>
        {children}
    </Reveal>;
}

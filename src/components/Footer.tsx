import githubLogo from '../assets/github-icon-1.svg'
import gitlabLogo from '../assets/gitlab-3.svg'

export const Footer = ()=>{
    return <div className="absolute w-full">
        <div className="bg-slate-600 text-white relative flex justify-center pt-6 pb-6 align-middle gap-4">
            <a href="https://github.com/SamTV12345" target="_blank" className="flex align-middle inline-block gap-3 hover:underline"><img src={githubLogo} alt="GitHub Logo" className="h-6"/><div className="hidden md:block">https://github.com/SamTV12345</div></a>
            <a href="https://gitlab.com/samuel1998.schwanzer" target="_blank" className="flex align-middle inline-block gap-3 hover:underline"><img src={gitlabLogo} alt="GitlabLogo Logo" className="h-6"/><div className="hidden md:block">https://gitlab.com/samuel1998.schwanzer</div></a>
    </div></div>
}

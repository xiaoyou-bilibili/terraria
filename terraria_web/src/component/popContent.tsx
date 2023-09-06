import {Input, Popconfirm, Toast} from "@douyinfe/semi-ui";
import {useState} from "react";
import {sendCommand} from "../api/api.tsx";

interface PopContentProps {
    title: string
    hint: string
    cmd: string
    children?: any
}

export function PopContent (props: PopContentProps) {
    // 当前输入的命令参数
    const [currentCmdParam, setCurrentCmdParam] = useState('')

    // 发送命令
    const sendCommandAction = (cmd: string) => {
        sendCommand(cmd).then(() => Toast.success("发送成功"))
    }

    return <Popconfirm
        title={props.title}
        onConfirm={() => sendCommandAction(`${props.cmd} ${currentCmdParam}`)}
        content={({}) => {
            return <Input onChange={setCurrentCmdParam} placeholder={props.hint} />;
        }}
    >
        {props.children}
    </Popconfirm>
}
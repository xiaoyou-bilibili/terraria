import {useEffect, useRef, useState} from 'react'
import './App.css'
import { Terminal } from 'xterm';
import { FitAddon } from 'xterm-addon-fit'; // xterm.js的插件，使终端的尺寸适合包含元素
import 'xterm/css/xterm.css'
import {
    Button,
    Card,
    Col,
    Descriptions, Form, Input,
    Modal, Popconfirm,
    Row,
    Space,
    TabPane,
    Tabs,
    Tag,
    Toast,
    Typography,
} from '@douyinfe/semi-ui';
import {editConfig, getEditConfig, sendCommand, startGame, stopGame} from "./api/api";
import {PopContent} from "./component/popContent.tsx";
import {GetEditConfResp} from "./api/model.tsx";
import {difficulty_size, word_size} from "./utils/const.tsx";

function App() {
    const { Text } = Typography;
    // 终端元素
    const term = useRef<HTMLDivElement|null>(null)
    // 终端
    const terminal = useRef<Terminal>()
    // websocket
    const socket = useRef<WebSocket>()
    // 当前的命令
    const command = useRef<String>("")
    // 编辑框是否可见
    const [editVisible, setEditVisible] = useState(false);
    // form对象
    const [formApi, setFormApi] = useState({getValues: ()=>Object})
    // 编辑信息
    const [confInfo, setConfInfo] = useState<GetEditConfResp>()
    // 服务密码
    const [serverPass, setServerPass] = useState("")

    // 发送命令
    const wsSendCommand = () => {
        socket.current?.send(`${command.current}\r\n`)
        // 回车时清空命令
        command.current = ""
    }

    const connect_socket = () => {
        if(!socket.current) {
            socket.current = new WebSocket(`ws://${window.location.host}/api/ws`)
            socket.current!.addEventListener("message", (e)=>{
                terminal.current?.write(`${e.data}\r`)
            })
        }
        fetchConfInfo()
    }

    useEffect(()=>{
        // 只初始化一次
        if(!terminal.current) {
            terminal.current = new Terminal()
            const fitAddon = new FitAddon();
            terminal.current?.loadAddon(fitAddon);
            terminal.current?.open(term.current!);
            terminal.current?.onKey(e=>{
                if(e.domEvent.code=='Enter') {
                    console.log(command.current)
                    terminal.current!.write("\r\n")
                    wsSendCommand()
                } else if (e.domEvent.code=='Backspace') {
                    terminal.current!.write('\b \b')
                    command.current = command.current.slice(0, -1)
                } else {
                    command.current += e.key
                    terminal.current!.write(e.key)
                }
            })
            fitAddon.fit();
        }
        fetchConfInfo()
    }, [])

    const editConfigAction = () => {
        setEditVisible(true)
    }

    const startGameAction = () => {
        setTimeout(connect_socket, 500)
        startGame().then(()=>{
            Toast.info("启动游戏成功")
        })
    }

    const stopGameAction = () => {
        stopGame().then(()=>Toast.info("关闭游戏成功"))
        setTimeout(() => {
            socket.current?.close()
            socket.current = undefined
            fetchConfInfo()
            clearTerminal()
        }, 500)
    }

    const saveEditAction = () => {
        editConfig(formApi.getValues()).then(() => setEditVisible(false))
    }

    // 获取配置文件
    const fetchConfInfo  = () => {
        getEditConfig().then((res:GetEditConfResp) => setConfInfo(res)).catch(() => {
            Toast.error("获取配置失败")
        })
    }

    // 发送命令
    const sendCommandAction = (cmd: string) => {
        sendCommand(cmd).then(() => Toast.success("发送成功"))
    }

    // 清空终端
    const clearTerminal = () => {
        terminal.current?.clear()
    }

    return (
    <>
        <Modal
            title="编辑游戏"
            visible={editVisible}
            onOk={saveEditAction}
            onCancel={()=>setEditVisible(false)}
        >
            <Form labelPosition='left' getFormApi={(form)=>setFormApi(form)}>
                <Form.Input field='word_name' label='世界名称'/>
                <Form.Input field='message' label='每日格言'/>
                <Form.RadioGroup field="word_size" label='世界大小'>
                    <Form.Radio value={1}>小</Form.Radio>
                    <Form.Radio value={2}>中</Form.Radio>
                    <Form.Radio value={3}>大</Form.Radio>
                </Form.RadioGroup>
                <Form.RadioGroup field="difficulty" label='难度'>
                    <Form.Radio value={0}>普通</Form.Radio>
                    <Form.Radio value={1}>专家</Form.Radio>
                    <Form.Radio value={2}>大师</Form.Radio>
                    <Form.Radio value={3}>旅途</Form.Radio>
                </Form.RadioGroup>
                <Form.Input field='password' label='密码'/>
                <Form.InputNumber field='port' label='端口' initValue={7777}/>
                <Form.InputNumber field='max_player' label='最大玩家数' initValue={7}/>
            </Form>
        </Modal>
        <Row gutter={[16, 16]}>
            <Col span={24}><div ref={term}></div></Col>
            <Col span={8}><Card><Descriptions data={[
                { key: '游戏状态', value: <Tag color={confInfo?.game_status?'green':'red'}>{confInfo?.game_status?'启动':'关闭'}</Tag> },
                { key: '端口', value:  <Text type="warning">{confInfo?.config.port}</Text>},
                { key: '默认密码', value: <Text type="danger">{confInfo?.config.password}</Text> },
                { key: '最大玩家数', value: confInfo?.config.max_player },
                { key: '每日消息', value: confInfo?.config.message },
                { key: '世界名称', value: confInfo?.config.word_name },
                { key: '世界大小', value: <Tag color={'cyan'}>{word_size.get(confInfo?.config.word_size || 0)}</Tag> },
                { key: '难度', value: <Tag color={'blue'}>{difficulty_size.get(confInfo?.config.difficulty || 0)}</Tag> },
            ]} /></Card></Col>
            <Col span={16}>
                <Tabs type="line">
                    <TabPane tab="基本控制" itemKey="1">
                        <Space>
                            <Button onClick={editConfigAction}>编辑世界</Button>
                            <Button onClick={startGameAction} type="secondary">开始游戏</Button>
                            <Button onClick={stopGameAction} type="danger">结束游戏</Button>
                            <Button onClick={clearTerminal} type="warning">清空终端</Button>
                            <Popconfirm
                                title={"设置服务密码"}
                                onConfirm={() => {
                                    localStorage.setItem("token", serverPass)
                                    Toast.success("设置成功")
                                    window.location.reload()
                                }}
                                content={({}) => {
                                    return <Input onChange={setServerPass} placeholder={"输入服务器的密码"} />;
                                }}
                            >
                                <Button type="primary">服务密码</Button>
                            </Popconfirm>
                        </Space>
                    </TabPane>
                    <TabPane tab="快捷命令" itemKey="2">
                        <Row gutter={[16, 16]}>
                            <Col><Space>
                                <Button theme='solid' onClick={()=>sendCommandAction("playing")}>显示玩家列表</Button>
                                <PopContent title={"踢人"} cmd={"kick"} hint={"玩家名"}>
                                    <Button theme='solid'>踢人</Button>
                                </PopContent>
                                <PopContent title={"禁止玩家"} cmd={"ban"} hint={"玩家名"}>
                                    <Button theme='solid'>禁止玩家</Button>
                                </PopContent>
                                <PopContent title={"发送全体消息"} cmd={"say"} hint={"消息内容"}>
                                    <Button theme='solid'>发送全体消息</Button>
                                </PopContent>
                            </Space></Col>
                        </Row>
                        <Row gutter={[16, 16]}>
                            <Col><Space>
                                <Button theme='solid' type='secondary' onClick={()=>sendCommandAction("password")}>显示密码</Button>
                                <PopContent title={"修改密码"} cmd={"password"} hint={"密码（关闭游戏后失效）"}>
                                    <Button theme='solid' type='secondary'>修改密码</Button>
                                </PopContent>
                                <Button theme='solid' type='secondary' onClick={()=>sendCommandAction("version")}>游戏版本号</Button>
                                <Button theme='solid' type='secondary' onClick={()=>sendCommandAction("settle")}>使所有的水平衡</Button>
                            </Space></Col>
                        </Row>
                        <Row gutter={[16, 16]}>
                            <Col><Space>
                                <Button theme='solid' type='warning' onClick={()=>sendCommandAction("time")}>游戏时间</Button>
                                <Button theme='solid' type='warning' onClick={()=>sendCommandAction("dawn")}>改成黎明</Button>
                                <Button theme='solid' type='warning' onClick={()=>sendCommandAction("noon")}>改成中午</Button>
                                <Button theme='solid' type='warning' onClick={()=>sendCommandAction("dusk")}>改成黄昏</Button>
                                <Button theme='solid' type='warning' onClick={()=>sendCommandAction("midnight")}>改成午夜</Button>
                            </Space></Col>
                        </Row>
                    </TabPane>
                </Tabs>
            </Col>
        </Row>
    </>
    )
}

export default App

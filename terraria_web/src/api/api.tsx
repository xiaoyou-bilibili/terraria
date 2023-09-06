// 修该配置
import request from "./request";
import {GetEditConfResp} from "./model.tsx";
const base = "/api"


// 配置相关
export const editConfig = (data: any) => request(`${base}/config`, data, 'post')
// 获取配置文件
export const getEditConfig = () => request<GetEditConfResp>(`${base}/config`, {}, 'get')
export const startGame = () => request(`${base}/game/start`, {}, 'get')
export const stopGame = () => request(`${base}/game/stop`, {}, 'get')
export const sendCommand = (cmd: string) => request(`${base}/game/cmd`, {cmd}, 'post')
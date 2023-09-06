import axios, {AxiosResponse} from 'axios'
import { Toast } from '@douyinfe/semi-ui';

const token = localStorage.getItem("token")

// 对axios函数进行封装，用来发api请求，post使用qs进行处理，避免自己把from数据转换为json字符串
export default async function request<T> (url:string, data:any, type:string) {
    let req:any
    // 判断请求类型
    if (type === 'get') {
        req = axios.get(url, { params: data, timeout: 1000 * 60 * 10, headers: {token} })
    } else if (type === 'post') {
        req = axios.post(url, data, {headers: {token}})
    } else if (type === 'put') {
        req = axios.put(url, data, {headers: {token}})
    } else if (type === 'delete') {
        req = axios.delete(url, {params: data, headers: {token}})
    } else if (type === 'patch') {
        req = axios.patch(url, data, {headers: {token}})
    }
    return new Promise<T>((resolve, reject) => {
        req.then((res: AxiosResponse) => {
            console.log(res)
            if (res.status !== 200) {
                if(res.status == 401) {
                    Toast.error("鉴权失败")
                } else {
                    Toast.error('请求失败')
                }
                reject('请求失败')
            }else if (res.data.code !== 0) {
                Toast.error(res.data.msg)
                reject(res.data.msg)
            } else {
                resolve(res.data.data)
            }
        })
    })
}
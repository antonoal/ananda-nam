import axios from 'axios'
import { useTokenStore } from '@/store/token'

const axiosClient = axios.create({
  baseURL: 'http://localhost:8080/api/1'
})

axiosClient.interceptors.request.use(
  (config) => {
    const token = useTokenStore().getToken()
    if (token) {
      config.headers.Authorization = `Bearer ${token}`
    }
    return config
  },
  (error) => {
    return Promise.reject(error)
  }
)

export default axiosClient

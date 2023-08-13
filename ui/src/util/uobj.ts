import {obj} from 'types'

export default {
    setDefault<T extends obj>(partial: Partial<T>, defaultValues: T): T {
        const response: obj = {}
        const keys = Object.keys(defaultValues)
        for (const key of keys) {
            response[key] = partial[key] ?? defaultValues[key]
        }
        return response as T
    }
}

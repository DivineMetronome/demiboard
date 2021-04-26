import humps from 'humps'
import {h} from "vue"

function styleToObject(style) {
    return style.split(';').map(s => s.trim()).filter(s => s).reduce((acc, pair) => {
        const i = pair.indexOf(':')
        const prop = humps.camelize(pair.slice(0, i))
        const value = pair.slice(i + 1).trim()

        acc[prop] = value

        return acc
    }, {})
}

function classToObject(cls) {
    return cls.split(/\s+/).reduce((acc, c) => {
        acc[c] = true

        return acc
    }, {})
}

function combineClassObjects(...objs) {
    return objs.reduce((acc, obj) => {
        if (Array.isArray(obj)) {
            acc = acc.concat(obj)
        } else {
            acc.push(obj)
        }

        return acc
    }, [])
}

function convert(element, props = {}, data = {}) {
    const children = (element.children || []).map(convert)
    const mixins = Object.keys(element.attributes || {}).reduce((acc, key) => {
        const val = element.attributes[key]

        switch (key) {
            case 'class':
                acc['class'] = classToObject(val)
                break
            case 'style':
                acc['style'] = styleToObject(val)
                break
            default:
                acc.attrs[key] = val
        }

        return acc
    }, {
        'class': {},
        style: {},
        attrs: {}
    })

    const {
        class: dClass = {},
        style: dStyle = {},
        attrs: dAttrs = {},
        ...remainingData
    } = Array.isArray(data) ? data[0] : data;

    if (typeof element === 'string') {
        return element
    } else {
        return h(element.tag, {
            class: combineClassObjects(mixins.class, dClass),
            style: {
                ... mixins.style,
                ...dStyle
            },
            ... mixins.attrs,
            ...dAttrs,
            ...remainingData,
            props
        }, children)
    }
}

export default convert

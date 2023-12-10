import en from '@/locales/en.json'
import ru from '@/locales/ru.json'
import { createI18n } from 'vue-i18n'

type MessageSchema = typeof en

const browserLocale = navigator.language.split('-')[0];

const i18n = createI18n<[MessageSchema], 'en' | 'ru'>({
  legacy: false,
  locale: browserLocale,
  fallbackLocale: 'en',
  messages: {
    en: en,
    ru: ru
  }
})

export default i18n
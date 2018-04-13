*[broadcast]* Сообщение для рассылки
{
  "embed": {
      "title": "Важная информация",
      "color": 37595,
      "fields": [
        {
          "name": "*WinSpirit*",
          "value": "[Группа ВК](https://vk.com/winspiritow)\n[Сайт](http://winspirit.org/)",
        "inline": true
        },
        {
          "name": "*Функции*",
          "value": "Просмотр статистики\nПоиск тиммейтов\nСписки турниров",
        "inline": true
        },

        {
          "name": "*Регулярные RU турниры и события*",
          "value": "[Cup of Dreamers](https://vk.com/cupofdreamers) - Регулярный турнир\n[PR1ME](https://vk.com/pr1mevk) - Турниры от стримеров\n[МИКСЫ](https://discord.gg/rDjtM59) - пон, сред, пятн. 20-00 МСК\n[ЛФГ сайт](http://oversearch.ru) - много игроков, гибкий фильтр",
          "inline": true
        },
        {
                  "name": "*Регулярные EU турниры*",
                  "value": "[OpenDivision](https://battlefy.com/overwatch-open-division-europe/2018-overwatch-open-division-season-2-europe/5ab2ee6f126cba034a112993/info?infoTab=details) - EU season 2 2018\n[Toornament.com](https://www.toornament.com/games/overwatch) - турнирная площадка\n[Battlefy.com](https://battlefy.com/browse/overwatch?region=Europe&platform=PC) - турнирная площадка\n[Go4OW](https://play.eslgaming.com/overwatch/europe/overwatch/major/go4overwatch-europe) - еженедельник с призовыми",
                  "inline": true
                },
        {
          "name": "*Инструментарий*",
          "value": "1. Совместный просмотр видео с возможностью рисовать - <https://visor.gg>\n2. Простой совместный просмоторщик видео - <https://andchill.tv>\n3. Общие таблицы для ведения статистики - <https://docs.google.com/>\n4. Общая тренировка аима - <http://store.steampowered.com/app/518030/Aim_Hero/>\n5. Настройка цветовой температуры на экране для оптимизации нагрузки на глаза - <https://justgetflux.com/>\n6. Энциклопедия овервотч - <https://overwiki.ru/>"

        },
        {
          "name": "*Записи игр | VODS*",
           "value": "Первая команда WS  - [100 часов, вид сверху, общение команды](https://www.youtube.com/channel/UCjMqWcQsXAXhI247M7knRYQ/playlists?view_as=subscriber)\nOWL и Contenders - [Официальные записи игр](https://vk.com/videos-39230591)\nEU&US VODS - [Ютуб канал с водами](https://www.youtube.com/channel/UC2J2ZrVtL_muVqK5xmiPyXw)"

                },
        {
          "name": "*На масло и улучшения*",
          "value": "PayPal - https://www.paypal.me/akseliter\nЯндекс кошелек - 41001266249359"
        }
        ,
        {
          "name": "__Важная информация обо мне__",
          "value": "Все списки поиска общие для всех серверов, так что вы можете установить меня к себе на сервер и пользоваться всеми моими плюшками с комфортом для себя =)\n[Ссылка для установки меня к вам на сервер!](https://discordapp.com/api/oauth2/authorize?client_id=291380685392838657&scope=bot&permissions=1)"
        },
        {
                  "name": "*Список команд* - !wsstats теперь обновляет роль по рейтингу на сервере WinSpirit",
                  "value": "```!wsreg BattleTag#1234 - регистрация для включения всех функций и выдача роли по рейтингу на сервере WinSpirit\n!wsstats - вывод статистики(если не вводил wsreg то необходимо указать BattleTag#1234)\n!wscmd - вывод полного списка команд\n!wshelp - вывод помощи и справочной информации```"
                }
      ]

    }
}

*[lfg_user_not_reg]* LFG незарегестрированный пользователь пробует воспользоваться lfg
{
  "embed": {
      "title": ":o: Упс..",
      "description": "Для данной функции требутся региcтрация",
      "color": 13369344,
      "footer": {
        "text": "Для помощи воспользуйтесь !wshelp"
      }
    }
}

*[lfg_user_no_btag]* LFG Необнаружен бтаг ни в сообщении, ни в профиле
{
  "embed": {
      "title": ":warning: Упс..",
      "description": "Вы не уазали BTag ни в профиле, ни при регистрации",
      "color": 13369344,
      "footer": {
        "text": "Для помощи воспользуйтесь !wshelp"
      }
    }
}

*[lfg_wrong_btag]* LFG по бтагу не найден профиль
{
  "embed": {
      "title": ":warning: Упс..",
      "description": "По указанной BTag-у и плтаформе не найден профиль Overwatch",
      "color": 13369344,
      "footer": {
        "text": "Для помощи воспользуйтесь !wshelp"
      }
    }
}

*[lfg_not_found_WTF]* LFG уже находил объявление но при взаимодействии не нашел. В принцие ошибка не должна появлятся но...
{
  "embed": {
      "title": "Упс..",
      "description": "Что-то пошло не так и вы не должны увидеть эту ошибку",
      "color": 13369344,
      "footer": {
        "text": "Просто не обращайте на меня внимание"
      }
    }
}

*[lfg_list_empty]* LFG никто не ищет комманду. Лист пуст
{
  "embed": {
      "title": "Лист пуст",
      "description": "К сожалению я не нашел никого, что бы вам показать",
      "color": 37595,
      "footer": {
        "text": "Но вы можете написать первым!"
      }
    }
}

*[lfg_del_notfound]* LFG пользователь похочет удалить объявление, но объявления и так нет
{
  "embed": {
      "title": ":warning: Упс..",
      "description": "За вами не закрепелено объявлений",
      "color": 13369344,
      "footer": {
        "text": "но вы можете создать, а потом удалить =3"
      }
    }
}

*[lfg_help]* LFG помощь по команде !wslfg
{
  "embed": {
    "title": "Привет я помошник для функций поиска игроков и команд",
    "url": "https://discordapp.com/oauth2/authorize?client_id=291380685392838657&scope=bot&permissions=1",
    "color": 37595,
    "image": {
      "url": "http://winspirit.org/sites/default/files/field/image/mrcy.jpg"
    },
    "fields": [
      {
        "name": "Полный набор команд из раздела lfg",
        "value": "```!wslfg BattleTag#1234 EU PC \"Ваше сообщение для списка поиска\" help del \n\nВсе вводится через пробел, а сообщение всегда в ковычках.```"
      },
      {
        "name": "!wslfg",
        "value": "Вывод списка игроков."
      },
      {
        "name": "!wslfg help",
        "value": "Вывод справки и помощи по функции поиска."
      },
      {
        "name": "!wslfg del",
        "value": "Удаление вашей записи из списка игроков. Возможно использование дополнительного синтаксиса delete rem remove."
      },
      {
        "name": "!wslfg \"Ваше сообщение\"",
        "value": "Добавление или обновление сообщения для списка. Для пользователей которые вводили команду !wsreg c батлтагом такой команды достаточно, чтобы внести себя в список игроков."
      },
      {
        "name": "!wslfg BattleTag#1234 eu pc",
        "value": "Команда для внесения вас в список поиска. Регион и платформу вводить необязательно, по умолчанию - eu pc"
      }
    ]
  }
}
*[cmd]* цмд !wslfg
{
  "embed": {
    "title": "Привет я помошник в освоении команд",
    "url": "https://discordapp.com/oauth2/authorize?client_id=291380685392838657&scope=bot&permissions=1",
    "color": 37595,
    "image": {
      "url": "http://winspirit.org/sites/default/files/ana_by_xyrlei-daqs404.jpg"
    },
    "fields": [
      {
        "name": "!wshelp",
        "value": "Вывод помощи."
      },
      {
        "name": "!wslfg",
        "value": "Вывод списка игроков."
      },
      {
        "name": "!wstour",
        "value": "Вывод списка туринров с открытой регистрацией"
      },
      {
        "name": "!wsreg BattleTag#1234 eu pc",
        "value": "!wsreg BattleTag#1234\nВсе вводится через пробел. Регион и платформу вводить необязательно, по умолчанию - eu pc.\nЯ сохраняю ваш БаттлеТаг, он необходим для некоторых моих сервисов и упрощает использования общих функций."
      },
      {
        "name": "!wsstats BattleTag#1234 eu pc",
        "value": "!wsstats BattleTag#1234 EU PC - выводит статистику игрока с overwatch.com\nРегион и платформу вводить необязательно, по умолчанию - eu pc. Если вы выполняли команду !wsreg, то можете не вводить БаттлТаг, я смогу взять его из сохранненого для вас."
      },
      {
        "name": "Полный набор команд из раздела lfg",
        "value": "!wslfg BattleTag#1234 EU PC \"Ваше сообщение для списка поиска\" help del\n!wslfg - Вывод списка игроков\n!wslfg help - Вывод справки по функции поиска\n!wslfg BattleTag#1234 \"Ищу тиму, танк, РТ 19-00 - 23-00 МСК\"\nКоманда для внесения вас в список поиска. Регион и платформу вводить необязательно, по умолчанию - eu pc"
      }
    ]
  }
}
*[tourneys]* Список турниров
{
  "embed": {
   "color": 37595,
    "description": "На все турниры в списке свободная регистрация\n====================================",
    "footer": {
      "text": "WinSpirit™"
    },
    "thumbnail": {
      "url": "http://winspirit.org/sites/default/files/full-quad-200px.png"
    },
    "image": {
      "url": "https://blizzard.gamespress.com/cdn/propressroom/Content/Artwork/Eva/BlizzardLive/artwork/2017/10/08170355-64c84909-3f5f-41b7-9dfc-39afdcaacfd2/OWContenders_S1_Playoffs_Day_One__(1).jpg?w=1024&maxheight=4096&mode=pad&format=jpg"
    },
    "author": {
      "name": "Список турниров"
      },
    "fields": [
      {
        "name": "EU # Open Division",
        "value": "Открытый турнир от Blizzard\n[подробнее...](https://battlefy.com/overwatch-open-division-europe)"
      },
            {
        "name": "EU # ESL Go4Overwatch",
        "value": "Еженедельный европейский турнир с сильным соперниками и небольшим призовым\n[подробнее...](https://play.eslgaming.com/overwatch/europe/overwatch/major/go4overwatch-europe)"
      },
      {
         "name": "RU # Meatgrinder",
        "value": "Еженедельник от WinSpirit\nскоро..."
      },
      {
         "name": "RU # Cup of Dreamers",
        "value": "Регулярный турнир от AlexanDream. 2 раза в год. Хороший призовой.\n[подробнее...](https://vk.com/cupofdreamers)"
      }
    ]
  }
}

*[help]* Помощь
{
  "embed": {
    "title": "Я омник, но не пугайся, я добрый омник =)",
    "description": "Я помогаю игрокам в OverWatch в разных аспектах профессиональной игры:",
    "url": "https://discordapp.com/api/oauth2/authorize?client_id=291380685392838657&scope=bot&permissions=1",
    "color": 37595,
    "thumbnail": {
      "url": "http://winspirit.org/sites/default/files/full-quad-200px.png"
    },
    "fields": [
      {
        "name": "*Получение статистики*",
        "value": "Я могу брать статистику\nс [playoverwatch.com](https://playoverwatch.com/en-us/)",
        "inline": true
      },
      {
        "name": "*Поиск тиммейтов и команд*",
        "value": "А также союзников для игры\nв аркады, рейтинг и быстрые игры",
        "inline": true
      },
      {
        "name": "*Оповещение о турнирах*",
        "value": "Актуальная информация\nо турнирах открытых\nдля участия",
        "inline": true
      },
      {
        "name": "*Помощь в развитии*",
        "value": "Личных навыков, командной игры,\nбаза знаний, конференции с\nпрофессиональными игроками",
        "inline": true
      },
      {
        "name": "*Инструментарий*",
        "value": "1. Совместный просмотр видео с возможностью рисовать - <https://visor.gg>\n2. Общие таблицы для ведения статистики - <https://docs.google.com/>\n3. Общая тренировка аима - <http://store.steampowered.com/app/518030/Aim_Hero/>\n4. Настройка цветовой температуры на экране для оптимизации нагрузки на глаза - <https://justgetflux.com/>"
                      },
      {
        "name": "*Ты можешь посетить наши ресурсы:*",
        "value": "http://winspirit.org/\nhttps://vk.com/winspiritow\n"
      },
      {
        "name": "*Краткий список команд*",
        "value": "!wscmd - полный и подробный список команд.\n!wsreg - внесение данных в БД для доступа к полному функционалу\n!wsstats - запрос статистики текущего соревновательного сезона"
      }
    ]
  }
}
*[mixruow]* Помощь
{
  "embed": {
   "color": 37595,
   "image": {
      "url": "https://pp.userapi.com/c834102/v834102304/111107/JiPUknsa2VA.jpg"

    },
    "author": {
      "name": "Начались миксы на RU overwatch"
      },
    "fields": [
      {
        "name": "Каждый понедельник, среду и пятницу в 20-00 по МСК",
        "value": "Собирается народ, создаёт лобби и набирают 2 команды из пришедших игроков. И катают.\n[Присоединиться](https://discord.gg/RTWnt)"
      }
    ]
  }
}
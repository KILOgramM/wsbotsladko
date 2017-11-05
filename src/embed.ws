*[broadcast]* Сообщение для рассылки
{
  "embed": {
      "color": 37595,
      "image": {
        "url": "http://winspirit.org/sites/default/files/findplayer_0.jpg"},
      "author": {
        "name": "Ссылка для установки меня к себе на сервер",
        "url": "https://discordapp.com/api/oauth2/authorize?client_id=291380685392838657&scope=bot&permissions=1",
        "icon_url": "http://winspirit.org/sites/default/files/full-quad-200px.png"
      },
      "fields": [
        {
          "name": "*WinSpirit*",
          "value": "[Группа ВК](https://vk.com/winspiritow)\n[Сайт](http://winspirit.org/)",
        "inline": true
        },
        {
          "name": "*Наши друзья*",
          "value": "[Новости](https://vk.com/cyberathleticow)\n[Стрим турниров](https://go.twitch.tv/tengu_ow)",
        "inline": true
        },
        {
          "name": "*Список команд*",
          "value": "```!wsreg BattleTag#1234 - регистрация для включения всех функций\n!wsstats - вывод статистики(если не вводил wsreg то необходимо указать BattleTag#1234)\n!wscmd - вывод полного списка команд\n!wshelp - вывод помощи и справочной информации\n!wslfg - вывод списка игроков в поиске, если ввести !wslfg \"Любое сообщение\", то попадаешь в список(работает только после wsreg)```"
        },
        {
          "name": "*Регулярные турниры*",
          "value": "[OpenDivision](https://overwatch.starladder.com/ru) - открытый турнир от Blizzard\n[Еженельник от ESL](https://play.eslgaming.com/overwatch/europe/) - европейский еженедельник с призовыми\n[Legendary Competitions of eSport](https://vk.com/lcescomp) - регулярный Российский турнир с хорошими призовыми"
        },
        {
          "name": "*Ищем таланты*",
          "value": "```1. Админов для проведения турниров.\n2. Админов для проведения регулярных миксов.\n3. И вообще если вы хотите помочь, то пишите KILOgramM'у в личку. Задачь на всех хватит!```"
        }
        ,
        {
          "name": "__Важная информация обо мне__",
          "value": "Все списки поиска общие для всех серверов, так что вы можете установить меня к себе на сервер и пользоваться всеми моими плюшками с комфортом для себя =)\n[Ссылка для установки меня к вам на сервер!](https://discordapp.com/api/oauth2/authorize?client_id=291380685392838657&scope=bot&permissions=1)"
        }
      ]

    }
}
*[cmd]* Список команд
{
"embed": {
   "title": "Привет я помошник в освоении команд",
   "url": "https://discordapp.com/api/oauth2/authorize?client_id=291380685392838657&scope=bot&permissions=1",
    "color": 37595,

    "image": {
      "url": "http://winspirit.org/sites/default/files/ana_by_xyrlei-daqs404.jpg"
    },

    "fields": [
      {
        "name": "!wshelp",
        "value": "```Вывод помощи.```",
        "inline": true
      },
      {
        "name": "!wscmd",
        "value": "```Вывод списка команд.```",
        "inline": true
      },
      {
        "name": "!wsreg BattleTag#1234 eu pc",
        "value": "```!wsreg BattleTag#1234\n\nВсе вводится через пробел. Регион и платформу вводить необязательно, по умолчанию - eu pc.\nЯ сохраняю ваш БаттлеТаг, он необходим для некоторых моих сервисов и упрощает использования общих функций.```"
      },
      {
        "name": "!wsstats BattleTag#1234 eu pc",
        "value": "```!wsstats BattleTag#1234 EU PC - выводит статистику игрока с overwatch.com\n\nРегион и платформу вводить необязательно, по умолчанию - eu pc. Если вы выполняли команду !wsreg, то можете не вводить БаттлТаг, я смогу взять его из сохранненого для вас.```"
      },
      {
        "name": "Полный набор команд из раздела lfg",
        "value": "```!wslfg BattleTag#1234 EU PC \"Ваше сообщение для списка поиска\" help del\n\nВсе вводится через пробел, а сообщение всегда в ковычках.```"
      },
      {
        "name": "!wslfg",
        "value": "```Вывод списка игроков.```",
        "inline": true
      },
      {
        "name": "!wslfg help",
        "value": "```Вывод справки по функции поиска.```",
        "inline": true
      },
      {
        "name": "!wslfg del",
        "value": "```Удаление вашей записи из списка игроков. Возможно использование дополнительного синтаксиса delete rem remove.```"
      },
      {
               "name": "!wslfg \"Ваше сообщение\"",
               "value": "```Добавление или обновление сообщения для списка. Для пользователей которые вводили команду !wsreg c батлтагом такой команды достаточно, чтобы внести себя в список игроков.```"
             },
             {
                     "name": "!wslfg BattleTag#1234 eu pc",
                     "value": "```Команда для внесения вас в список поиска. Регион и платформу вводить необязательно, по умолчанию - eu pc.```"
                   }

    ]
}
}
*[lfg_user_not_reg]* LFG незарегестрированный пользователь пробует воспользоваться lfg

*[lfg_user_no_btag]* LFG Необнаружен бтаг ни в сообщении, ни в профиле

*[lfg_wrong_btag]* LFG по бтагу не найден профиль

*[lfg_not_found_WTF]* LFG уже находил объявление но при взаимодействии не нашел. В принцие ошибка не должна появлятся но...

*[lfg_list_empty]* LFG никто не ищет комманду. Лист пуст

*[lfg_del_notfound]* LFG пользователь похочет удалить объявление, но объявления и так нет

*[lfg_help]* LFG помощь по команде !wslfg
{
  "embed": {
    "title": "Привет я помошник для функций поиска игроков и команд",
   "url": "https://discordapp.com/api/oauth2/authorize?client_id=291380685392838657&scope=bot&permissions=1",
    "color": 37595,

    "image": {
      "url": "http://winspirit.org/sites/default/files/field/image/mrcy.jpg"
    },

    "fields": [
      {
        "name": "Полный набор команд из раздела lfg",
        "value": "```!wslfg BattleTag#1234 EU PC \"Ваше сообщение для списка поиска\" help del\n\nВсе вводится через пробел, а сообщение всегда в ковычках.```"
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

*[tourneys]* Список турниров
{
  "embed": {
   "color": 37595,
    "description": "На все турниры в списке открыта регистрация\n====================================",
    "timestamp": "2017-10-16T23:36:52.898Z",
    "footer": {
      "text": "WinSpirit™"
    },

    "image": {
      "url": "https://blizzard.gamespress.com/cdn/propressroom/Content/Artwork/Eva/BlizzardLive/artwork/2017/10/08170355-64c84909-3f5f-41b7-9dfc-39afdcaacfd2/OWContenders_S1_Playoffs_Day_One__(1).jpg?w=1024&maxheight=4096&mode=pad&format=jpg"
    },
    "author": {
      "name": "Список турниров"
      },
    "fields": [
      {
        "name": "# Open Division",
        "value": "Открытый турнир от Blizzard и StarLadder\n[подробнее...](https://overwatch.starladder.com/ru/season3)"
      },
      {
         "name": "# LCES [Legendary Competitions of eSport]",
        "value": "Российский регулярный онлайн турнир с хорошими призовыми\n[подробнее...](https://vk.com/lcescomp)"
      },
      {
        "name": "# ESL Go4Overwatch",
        "value": "Еженедельный европейский турнир с сильным соперниками и небольшим призовым\n[подробнее...](https://play.eslgaming.com/overwatch/europe/overwatch/major/go4overwatch-europe/cup-73/)"
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

    "image": {
      "url": "https://i.mycdn.me/image?id=837867512990&t=0&plc=WEB&tkn=*TWC36ZePKUyxlXxZGFM0HDrGYpg"
    },
    "fields": [
      {
        "name": "Получение статистики",
        "value": "Я могу брать статистику\nс [playoverwatch.com](https://playoverwatch.com/en-us/)",
        "inline": true
      },
      {
        "name": "Поиск тиммейтов и команд",
        "value": "А также союзников для игры\nв аркады, рейтинг и быстрые игры",
        "inline": true
      },
      {
        "name": "Оповещение о турнирах",
        "value": "Актуальная информация\nо турнирах открытых\nдля участия",
        "inline": true
      },
      {
        "name": "Помощь в развитии",
        "value": "Личных навыков, командной игры,\nбаза знаний, конференции с\nпрофессиональными игроками",
        "inline": true
      },
      {
        "name": "Ты можешь посетить наши ресурсы:",
        "value": "http://winspirit.org/\nhttps://discord.gg/CRfDBkX\nhttps://vk.com/winspiritow\n"
      },
      {
        "name": "Краткий список команд",
        "value": "!wscmd - полный и подробный список команд.\n!wsreg - внесение данных в БД для доступа к полному функционалу\n!wsstats - запрос статистики текущего соревновательного сезона"
      }
    ]
  }
}
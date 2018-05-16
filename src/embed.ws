*[broadcast]* Сообщение для рассылки
{
   "embed": {
    "title": "Самое важное, что вы должны знать о WinSpirit",
    "description": "Мы работаем каждый день - наша цель развивать сообщество по Overwatch",
    "color": 37595,
    "footer": {
      "text": "Удачи и удовольствия!"
    },
    "fields": [
    {
        "name": "Хочешь реального скилла?",
        "value": "У нас есть сетка событий - http://winspirit.org/events, каналы для поиска игроков для разных целей и поиск тренировок, каналы с огромным количеством стримов и ссылок на полезную информацию"
      },
      {
        "name": "ПОКУПАЙ ПОДПИСКУ!",
        "value": "Ты получишь возможность задать вопросым лучшим русcкоязычным игрокам. Пройдя по ссылке - http://www.donationalerts.ru/r/winspirit, укажите ПОДПИСКА и ваше дискорд имя. Стоимость подписки - 500 рублей за 31 день, 2500 за 183 дня и 4000 за 365 дней. Акция **Первый Шаг** только до конца мая стоимость подписки - 100 рублей."
      },
      {
        "name": "Что будет!?",
        "value": "Вы сможете не просто слушать спикеров, но и задавать вопросы. Так же на вырученные деньги мы будем увеличивать количество спикеров. Сделаем еженедельный, ежемесячные финалы и годовой итоговый турнир. Так же хотим организовать студию для качественных трансляций всех событий из мира Overwatch."
       },
      {
        "name": "ВАРИАНТЫ",
        "value": "Все, что можно было сделать бесплатного мы сделали и сейчас нам надо для дальнейшего развития или зажимать гайки и ограничевать вас на сервере или все-таки убедить вас помогать проекту. Заранее благодарим Вас за поддержку и понимание."
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
               "name": "RU # Турнир Жестяного Кубка",
              "value": "Очень можный турнир от WinSpirit с открытой регистрацией\n[подробнее...](http://winspirit.org/jestcup)"
            },
      {
         "name": "RU # Meatgrinder",
        "value": "Еженедельник от WinSpirit\nскоро..."
      },
      {
                "name": "RU # Cup of Dreamers",
               "value": "Регулярный турнир от AlexanDream. 2 раза в год. Хороший призовой.\n[подробнее...](https://vk.com/cupofdreamers)"
             },
      {
               "name": "RU # LCES - Legendary Competitions of eSport",
               "value": "Регулярный турнир. Хороший призовой. LAN турниры в Санкт-Петербурге\n[подробнее...](https://vk.com/lcescomp)"
             },
             {
                            "name": "RU # OK Challenge Overwatch",
                            "value": "Регулярный онлайн турнир. Призовой 100 000 рублей.\n[подробнее...](https://vk.com/okchallenge)"
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
        "value": "1. Совместный просмотр видео с возможностью рисовать - <https://visor.gg>\n2. Простой совместный просмоторщик видео - <https://andchill.tv>\n3. Общие таблицы для ведения статистики - <https://docs.google.com/>\n4. Общая тренировка аима - <http://store.steampowered.com/app/518030/Aim_Hero/>\n5. Настройка цветовой температуры на экране для оптимизации нагрузки на глаза - <https://justgetflux.com/>\n6. Энциклопедия овервотч - <https://overwiki.ru/>\n7. Подробная энциклопедия профессионального Overwatch - <http://liquipedia.net/overwatch/Main_Page>\n8. Хороший и полезный сайт от Таверны по Overwatch - <https://overwatch.tavernofheroes.net/ru>"
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
*[mixruow]* миксы
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
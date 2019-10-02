import Telegraf from 'telegraf';
import express from 'express';
import pg from 'pg';
import dotenv from 'dotenv';
import morgan from 'morgan';
dotenv.config();

const app = new Telegraf(process.env.BOT_TOKEN);
var logger = morgan('combined');
app.context.db = new pg.Client();
app.context.db.connect()
app.hears(/\/balance(.*)/, async ctx => {
  if(ctx.match[1] == "") {
    let {username} = ctx.update.message.from;
  } else {
    let username = ctx.match[1];
  }

  ctx.reply(`${username}'s balance is ?`);
});
const expressApp = express()
expressApp.use(app.webhookCallback('/ellipticoin-telegam-bot'))
expressApp.use(logger);
expressApp.listen(process.env.PORT || 8080, () => {
  console.log('Example app listening on port 8080!')
})

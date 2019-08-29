import Telegraf from 'telegraf';
import express from 'express';
import pg from 'pg';
import dotenv from 'dotenv';
dotenv.config();

const app = new Telegraf(process.env.BOT_TOKEN);
app.context.db = new pg.Client();
app.context.db.connect()
app.hears('/balance', async ctx => {
  const res = await ctx.db.query('SELECT $1::text as message', ['Hello world!'])
  console.log(res.rows[0].message)
  ctx.reply(res.rows[0].message);
});
const expressApp = express()
expressApp.use(app.webhookCallback('/ellipticoin-telegam-bot'))
expressApp.listen(process.env.PORT || 8080, () => {
  console.log('Example app listening on port 8080!')
})

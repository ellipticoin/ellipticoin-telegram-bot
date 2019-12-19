import Long from 'long';
import Telegraf from 'telegraf';
import base64url from 'base64url';
import dotenv from 'dotenv';
import ec from 'ec-client';
import ethers from 'ethers';
import express from 'express';
import fs from 'fs';
import os from "os";
import morgan from 'morgan';

dotenv.config();

const EXCHANGE_RATE = 10000000 / 1;
const EC_SCALE = 10000;
const ELLIPTICOIN_ADDRESS = Buffer("hLIY8BEfVC93S1rlx99ABhtVr8XPjyEhkjCU3Z9qucw=", "base64");
const BOT_CONTRACT = new ec.Contract(ELLIPTICOIN_ADDRESS, "TelegramBot");
const CONTRACT_NAME = "TelegramBot";
const app = new Telegraf(process.env.BOT_TOKEN);
const DAI_CONTRACT_ADDRESS = "0x92d13090891f5bf92fa7f275a9a69df5e566fb84";
const INFURA = new ethers.providers.InfuraProvider('kovan');
const DAI_ABI = JSON.parse(fs.readFileSync("mintableTokenABI.json", "utf8"))
const WALLET = new ethers.Wallet(process.env.ETHEREUM_PRIVATE_KEY, INFURA);
const CLIENT = ec.Client.fromConfig(`${os.homedir()}/.ec-wallet/config.yaml`);
let DAI = new ethers.Contract(DAI_CONTRACT_ADDRESS, DAI_ABI, WALLET);
var logger = morgan('combined');
app.hears(/\/balance ?@?(.*)/, async ctx => {
  let username;
  if(ctx.match[1] == "") {
    username= ctx.update.message.from.username;
  } else {
    username = ctx.match[1];
  }

  ctx.reply(`@${username}'s balance is ${await balanceOf(username)}`);
});

app.hears(/\/transfer @?(.*) (.*)/, async ctx => {
  let from = ctx.update.message.from.username;
  let to = ctx.match[1];
  let amount = parseFloat(ctx.match[2]);
  ctx.replyWithMarkdown(`Transferring ${amount} EC to ${to}`);
  let transaction = await transfer(from, to, amount * EC_SCALE);
  let completedTransaction = await CLIENT.waitForTransactionToBeMined(transaction);
  if (completedTransaction.return_code != 0) {
    ctx.reply(`Smart contract error: ${completedTransaction.return_value}`);
  }
  ctx.replyWithMarkdown(transactionLink(ec.objectHash(transaction)));
});

app.hears(/\/cashout (.*) (.*)/, async ctx => {
  let username = ctx.update.message.from.username;
  let ECAmount = parseFloat(ctx.match[1]);
  let to = ctx.match[2];
  let DAIAmount = ECAmount * EC_SCALE / EXCHANGE_RATE;
  ctx.reply(`Cashing out ${ECAmount} EC for ${DAIAmount} DAI to ${to}`);
  let transaction = await withdraw(username, ECAmount * EC_SCALE);
  let completedTransaction = await CLIENT.waitForTransactionToBeMined(transaction);
  ctx.replyWithMarkdown(transactionLink(ec.objectHash(transaction)), Telegraf.Extra.webPreview(false));
  let tx = await sendDai(to, DAIAmount);
  ctx.replyWithMarkdown(etherscanLink(tx.hash), Telegraf.Extra.webPreview(false));

})

async function transfer(from, to, amount) {
  const transaction = BOT_CONTRACT.createTransaction(
      "transfer",
      new Buffer(32),
      "Ellipticoin",
      from,
      to,
      amount,
  );
  return await CLIENT.post(transaction);
}

async function withdraw(from, amount) {
  const transaction = BOT_CONTRACT.createTransaction(
      "withdraw",
      new Buffer(32),
      "Ellipticoin",
      from,
      amount,
  );
  return await CLIENT.post(transaction);
}

async function sendDai(to, amount){
  return await DAI.transfer(to, ethers.utils.parseEther(amount.toString(), "eth"))
};

const transactionLink = (transactionHash) =>
`[View Ellipticoin Transaction](https://block-explorer.ellipticoin.org/transactions/${base64url(transactionHash)})`

const etherscanLink = (transactionHash) =>
`[View Ethereum Transaction on Etherscan](https://kovan.etherscan.io/tx/${transactionHash})`

async function balanceOf(username) {
    let key = Buffer.concat([
      new Buffer(32),
      Buffer.from("Ellipticoin", "utf8"),
      Buffer.from(username, "utf8"),
    ]);
    let value = await CLIENT.getMemory(ELLIPTICOIN_ADDRESS, CONTRACT_NAME, key)
    if(value) {
      return (bytesToNumber(value)/EC_SCALE).toFixed(4);
    } else {
      return 0;
    }
}

export function bytesToNumber(bytes) {
  return Long.fromBytesLE(Buffer.from(bytes)).toNumber()
}

const expressApp = express()
expressApp.use(logger);
expressApp.use(app.webhookCallback('/ellipticoin-telegam-bot'))
expressApp.listen(process.env.PORT || 8081, () => {
  console.log('Example app listening on port 8081!')
})

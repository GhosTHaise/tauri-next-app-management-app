import React from 'react'
import styles from "./Hero.module.css";
import Image from 'next/image';

const Hero = () => {
  return (
    <div className={styles["weather_app"]} style={{backgroundImage : "url('/assets/Day/cloudy.jpg')"}}>
        <div className={styles.container}>
            <h3 className={styles.brand}>
                The weather
            </h3>
            <div>
                <h1 className={styles.temp}>
                    16&#176;
                </h1>
                <div className={styles.city_time}>
                    <h1 className={styles.name}>London</h1>
                    <small>
                        <span className={styles.time}>06:09</span>
                        -
                        <span className={styles.date}>Monday Sep 19</span>
                    </small>
                </div>
                <div className={styles.weather}>
                    <Image 
                        src='/Weather/day/113.png'
                        className={styles.icon}
                        alt="icon"
                        width={50}
                        height={50}
                    />
                    <span className={styles.condition}>
                        Cloudy
                    </span>
                </div>
            </div>
        </div>
    </div>
  )
}

export default Hero
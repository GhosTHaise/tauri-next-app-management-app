import React from 'react'
import styles from "./Hero.module.css";

const Hero = () => {
  return (
    <div className={styles.weather_app}>
        <div className={styles.container}>
            <h3>
                The weather
            </h3>
        </div>
    </div>
  )
}

export default Hero
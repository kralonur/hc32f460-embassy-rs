#[doc = "Register `FCG2` reader"]
pub type R = crate::R<Fcg2Spec>;
#[doc = "Register `FCG2` writer"]
pub type W = crate::W<Fcg2Spec>;
#[doc = "Field `TIMER0_1` reader - desc TIMER0_1"]
pub type Timer0_1R = crate::BitReader;
#[doc = "Field `TIMER0_1` writer - desc TIMER0_1"]
pub type Timer0_1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER0_2` reader - desc TIMER0_2"]
pub type Timer0_2R = crate::BitReader;
#[doc = "Field `TIMER0_2` writer - desc TIMER0_2"]
pub type Timer0_2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMERA_1` reader - desc TIMERA_1"]
pub type Timera1R = crate::BitReader;
#[doc = "Field `TIMERA_1` writer - desc TIMERA_1"]
pub type Timera1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMERA_2` reader - desc TIMERA_2"]
pub type Timera2R = crate::BitReader;
#[doc = "Field `TIMERA_2` writer - desc TIMERA_2"]
pub type Timera2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMERA_3` reader - desc TIMERA_3"]
pub type Timera3R = crate::BitReader;
#[doc = "Field `TIMERA_3` writer - desc TIMERA_3"]
pub type Timera3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMERA_4` reader - desc TIMERA_4"]
pub type Timera4R = crate::BitReader;
#[doc = "Field `TIMERA_4` writer - desc TIMERA_4"]
pub type Timera4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMERA_5` reader - desc TIMERA_5"]
pub type Timera5R = crate::BitReader;
#[doc = "Field `TIMERA_5` writer - desc TIMERA_5"]
pub type Timera5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMERA_6` reader - desc TIMERA_6"]
pub type Timera6R = crate::BitReader;
#[doc = "Field `TIMERA_6` writer - desc TIMERA_6"]
pub type Timera6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER4_1` reader - desc TIMER4_1"]
pub type Timer4_1R = crate::BitReader;
#[doc = "Field `TIMER4_1` writer - desc TIMER4_1"]
pub type Timer4_1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER4_2` reader - desc TIMER4_2"]
pub type Timer4_2R = crate::BitReader;
#[doc = "Field `TIMER4_2` writer - desc TIMER4_2"]
pub type Timer4_2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER4_3` reader - desc TIMER4_3"]
pub type Timer4_3R = crate::BitReader;
#[doc = "Field `TIMER4_3` writer - desc TIMER4_3"]
pub type Timer4_3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMB` reader - desc EMB"]
pub type EmbR = crate::BitReader;
#[doc = "Field `EMB` writer - desc EMB"]
pub type EmbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER6_1` reader - desc TIMER6_1"]
pub type Timer6_1R = crate::BitReader;
#[doc = "Field `TIMER6_1` writer - desc TIMER6_1"]
pub type Timer6_1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER6_2` reader - desc TIMER6_2"]
pub type Timer6_2R = crate::BitReader;
#[doc = "Field `TIMER6_2` writer - desc TIMER6_2"]
pub type Timer6_2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER6_3` reader - desc TIMER6_3"]
pub type Timer6_3R = crate::BitReader;
#[doc = "Field `TIMER6_3` writer - desc TIMER6_3"]
pub type Timer6_3W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc TIMER0_1"]
    #[inline(always)]
    pub fn timer0_1(&self) -> Timer0_1R {
        Timer0_1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc TIMER0_2"]
    #[inline(always)]
    pub fn timer0_2(&self) -> Timer0_2R {
        Timer0_2R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc TIMERA_1"]
    #[inline(always)]
    pub fn timera_1(&self) -> Timera1R {
        Timera1R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc TIMERA_2"]
    #[inline(always)]
    pub fn timera_2(&self) -> Timera2R {
        Timera2R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc TIMERA_3"]
    #[inline(always)]
    pub fn timera_3(&self) -> Timera3R {
        Timera3R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc TIMERA_4"]
    #[inline(always)]
    pub fn timera_4(&self) -> Timera4R {
        Timera4R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc TIMERA_5"]
    #[inline(always)]
    pub fn timera_5(&self) -> Timera5R {
        Timera5R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc TIMERA_6"]
    #[inline(always)]
    pub fn timera_6(&self) -> Timera6R {
        Timera6R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - desc TIMER4_1"]
    #[inline(always)]
    pub fn timer4_1(&self) -> Timer4_1R {
        Timer4_1R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc TIMER4_2"]
    #[inline(always)]
    pub fn timer4_2(&self) -> Timer4_2R {
        Timer4_2R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - desc TIMER4_3"]
    #[inline(always)]
    pub fn timer4_3(&self) -> Timer4_3R {
        Timer4_3R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 15 - desc EMB"]
    #[inline(always)]
    pub fn emb(&self) -> EmbR {
        EmbR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - desc TIMER6_1"]
    #[inline(always)]
    pub fn timer6_1(&self) -> Timer6_1R {
        Timer6_1R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - desc TIMER6_2"]
    #[inline(always)]
    pub fn timer6_2(&self) -> Timer6_2R {
        Timer6_2R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - desc TIMER6_3"]
    #[inline(always)]
    pub fn timer6_3(&self) -> Timer6_3R {
        Timer6_3R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc TIMER0_1"]
    #[inline(always)]
    pub fn timer0_1(&mut self) -> Timer0_1W<'_, Fcg2Spec> {
        Timer0_1W::new(self, 0)
    }
    #[doc = "Bit 1 - desc TIMER0_2"]
    #[inline(always)]
    pub fn timer0_2(&mut self) -> Timer0_2W<'_, Fcg2Spec> {
        Timer0_2W::new(self, 1)
    }
    #[doc = "Bit 2 - desc TIMERA_1"]
    #[inline(always)]
    pub fn timera_1(&mut self) -> Timera1W<'_, Fcg2Spec> {
        Timera1W::new(self, 2)
    }
    #[doc = "Bit 3 - desc TIMERA_2"]
    #[inline(always)]
    pub fn timera_2(&mut self) -> Timera2W<'_, Fcg2Spec> {
        Timera2W::new(self, 3)
    }
    #[doc = "Bit 4 - desc TIMERA_3"]
    #[inline(always)]
    pub fn timera_3(&mut self) -> Timera3W<'_, Fcg2Spec> {
        Timera3W::new(self, 4)
    }
    #[doc = "Bit 5 - desc TIMERA_4"]
    #[inline(always)]
    pub fn timera_4(&mut self) -> Timera4W<'_, Fcg2Spec> {
        Timera4W::new(self, 5)
    }
    #[doc = "Bit 6 - desc TIMERA_5"]
    #[inline(always)]
    pub fn timera_5(&mut self) -> Timera5W<'_, Fcg2Spec> {
        Timera5W::new(self, 6)
    }
    #[doc = "Bit 7 - desc TIMERA_6"]
    #[inline(always)]
    pub fn timera_6(&mut self) -> Timera6W<'_, Fcg2Spec> {
        Timera6W::new(self, 7)
    }
    #[doc = "Bit 8 - desc TIMER4_1"]
    #[inline(always)]
    pub fn timer4_1(&mut self) -> Timer4_1W<'_, Fcg2Spec> {
        Timer4_1W::new(self, 8)
    }
    #[doc = "Bit 9 - desc TIMER4_2"]
    #[inline(always)]
    pub fn timer4_2(&mut self) -> Timer4_2W<'_, Fcg2Spec> {
        Timer4_2W::new(self, 9)
    }
    #[doc = "Bit 10 - desc TIMER4_3"]
    #[inline(always)]
    pub fn timer4_3(&mut self) -> Timer4_3W<'_, Fcg2Spec> {
        Timer4_3W::new(self, 10)
    }
    #[doc = "Bit 15 - desc EMB"]
    #[inline(always)]
    pub fn emb(&mut self) -> EmbW<'_, Fcg2Spec> {
        EmbW::new(self, 15)
    }
    #[doc = "Bit 16 - desc TIMER6_1"]
    #[inline(always)]
    pub fn timer6_1(&mut self) -> Timer6_1W<'_, Fcg2Spec> {
        Timer6_1W::new(self, 16)
    }
    #[doc = "Bit 17 - desc TIMER6_2"]
    #[inline(always)]
    pub fn timer6_2(&mut self) -> Timer6_2W<'_, Fcg2Spec> {
        Timer6_2W::new(self, 17)
    }
    #[doc = "Bit 18 - desc TIMER6_3"]
    #[inline(always)]
    pub fn timer6_3(&mut self) -> Timer6_3W<'_, Fcg2Spec> {
        Timer6_3W::new(self, 18)
    }
}
#[doc = "desc FCG2\n\nYou can [`read`](crate::Reg::read) this register and get [`fcg2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcg2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fcg2Spec;
impl crate::RegisterSpec for Fcg2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fcg2::R`](R) reader structure"]
impl crate::Readable for Fcg2Spec {}
#[doc = "`write(|w| ..)` method takes [`fcg2::W`](W) writer structure"]
impl crate::Writable for Fcg2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FCG2 to value 0xffff_ffff"]
impl crate::Resettable for Fcg2Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}

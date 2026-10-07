#[doc = "Register `RCSR` reader"]
pub type R = crate::R<RcsrSpec>;
#[doc = "Register `RCSR` writer"]
pub type W = crate::W<RcsrSpec>;
#[doc = "Field `RTIDU` reader - desc RTIDU"]
pub type RtiduR = crate::BitReader;
#[doc = "Field `RTIDU` writer - desc RTIDU"]
pub type RtiduW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTIDV` reader - desc RTIDV"]
pub type RtidvR = crate::BitReader;
#[doc = "Field `RTIDV` writer - desc RTIDV"]
pub type RtidvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTIDW` reader - desc RTIDW"]
pub type RtidwR = crate::BitReader;
#[doc = "Field `RTIDW` writer - desc RTIDW"]
pub type RtidwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTIFU` reader - desc RTIFU"]
pub type RtifuR = crate::BitReader;
#[doc = "Field `RTICU` reader - desc RTICU"]
pub type RticuR = crate::BitReader;
#[doc = "Field `RTICU` writer - desc RTICU"]
pub type RticuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTEU` reader - desc RTEU"]
pub type RteuR = crate::BitReader;
#[doc = "Field `RTEU` writer - desc RTEU"]
pub type RteuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTSU` reader - desc RTSU"]
pub type RtsuR = crate::BitReader;
#[doc = "Field `RTSU` writer - desc RTSU"]
pub type RtsuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTIFV` reader - desc RTIFV"]
pub type RtifvR = crate::BitReader;
#[doc = "Field `RTICV` reader - desc RTICV"]
pub type RticvR = crate::BitReader;
#[doc = "Field `RTICV` writer - desc RTICV"]
pub type RticvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTEV` reader - desc RTEV"]
pub type RtevR = crate::BitReader;
#[doc = "Field `RTEV` writer - desc RTEV"]
pub type RtevW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTSV` reader - desc RTSV"]
pub type RtsvR = crate::BitReader;
#[doc = "Field `RTSV` writer - desc RTSV"]
pub type RtsvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTIFW` reader - desc RTIFW"]
pub type RtifwR = crate::BitReader;
#[doc = "Field `RTICW` reader - desc RTICW"]
pub type RticwR = crate::BitReader;
#[doc = "Field `RTICW` writer - desc RTICW"]
pub type RticwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTEW` reader - desc RTEW"]
pub type RtewR = crate::BitReader;
#[doc = "Field `RTEW` writer - desc RTEW"]
pub type RtewW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTSW` reader - desc RTSW"]
pub type RtswR = crate::BitReader;
#[doc = "Field `RTSW` writer - desc RTSW"]
pub type RtswW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc RTIDU"]
    #[inline(always)]
    pub fn rtidu(&self) -> RtiduR {
        RtiduR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc RTIDV"]
    #[inline(always)]
    pub fn rtidv(&self) -> RtidvR {
        RtidvR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc RTIDW"]
    #[inline(always)]
    pub fn rtidw(&self) -> RtidwR {
        RtidwR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - desc RTIFU"]
    #[inline(always)]
    pub fn rtifu(&self) -> RtifuR {
        RtifuR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc RTICU"]
    #[inline(always)]
    pub fn rticu(&self) -> RticuR {
        RticuR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc RTEU"]
    #[inline(always)]
    pub fn rteu(&self) -> RteuR {
        RteuR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc RTSU"]
    #[inline(always)]
    pub fn rtsu(&self) -> RtsuR {
        RtsuR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - desc RTIFV"]
    #[inline(always)]
    pub fn rtifv(&self) -> RtifvR {
        RtifvR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc RTICV"]
    #[inline(always)]
    pub fn rticv(&self) -> RticvR {
        RticvR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - desc RTEV"]
    #[inline(always)]
    pub fn rtev(&self) -> RtevR {
        RtevR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - desc RTSV"]
    #[inline(always)]
    pub fn rtsv(&self) -> RtsvR {
        RtsvR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - desc RTIFW"]
    #[inline(always)]
    pub fn rtifw(&self) -> RtifwR {
        RtifwR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - desc RTICW"]
    #[inline(always)]
    pub fn rticw(&self) -> RticwR {
        RticwR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - desc RTEW"]
    #[inline(always)]
    pub fn rtew(&self) -> RtewR {
        RtewR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - desc RTSW"]
    #[inline(always)]
    pub fn rtsw(&self) -> RtswR {
        RtswR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc RTIDU"]
    #[inline(always)]
    pub fn rtidu(&mut self) -> RtiduW<'_, RcsrSpec> {
        RtiduW::new(self, 0)
    }
    #[doc = "Bit 1 - desc RTIDV"]
    #[inline(always)]
    pub fn rtidv(&mut self) -> RtidvW<'_, RcsrSpec> {
        RtidvW::new(self, 1)
    }
    #[doc = "Bit 2 - desc RTIDW"]
    #[inline(always)]
    pub fn rtidw(&mut self) -> RtidwW<'_, RcsrSpec> {
        RtidwW::new(self, 2)
    }
    #[doc = "Bit 5 - desc RTICU"]
    #[inline(always)]
    pub fn rticu(&mut self) -> RticuW<'_, RcsrSpec> {
        RticuW::new(self, 5)
    }
    #[doc = "Bit 6 - desc RTEU"]
    #[inline(always)]
    pub fn rteu(&mut self) -> RteuW<'_, RcsrSpec> {
        RteuW::new(self, 6)
    }
    #[doc = "Bit 7 - desc RTSU"]
    #[inline(always)]
    pub fn rtsu(&mut self) -> RtsuW<'_, RcsrSpec> {
        RtsuW::new(self, 7)
    }
    #[doc = "Bit 9 - desc RTICV"]
    #[inline(always)]
    pub fn rticv(&mut self) -> RticvW<'_, RcsrSpec> {
        RticvW::new(self, 9)
    }
    #[doc = "Bit 10 - desc RTEV"]
    #[inline(always)]
    pub fn rtev(&mut self) -> RtevW<'_, RcsrSpec> {
        RtevW::new(self, 10)
    }
    #[doc = "Bit 11 - desc RTSV"]
    #[inline(always)]
    pub fn rtsv(&mut self) -> RtsvW<'_, RcsrSpec> {
        RtsvW::new(self, 11)
    }
    #[doc = "Bit 13 - desc RTICW"]
    #[inline(always)]
    pub fn rticw(&mut self) -> RticwW<'_, RcsrSpec> {
        RticwW::new(self, 13)
    }
    #[doc = "Bit 14 - desc RTEW"]
    #[inline(always)]
    pub fn rtew(&mut self) -> RtewW<'_, RcsrSpec> {
        RtewW::new(self, 14)
    }
    #[doc = "Bit 15 - desc RTSW"]
    #[inline(always)]
    pub fn rtsw(&mut self) -> RtswW<'_, RcsrSpec> {
        RtswW::new(self, 15)
    }
}
#[doc = "desc RCSR\n\nYou can [`read`](crate::Reg::read) this register and get [`rcsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RcsrSpec;
impl crate::RegisterSpec for RcsrSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`rcsr::R`](R) reader structure"]
impl crate::Readable for RcsrSpec {}
#[doc = "`write(|w| ..)` method takes [`rcsr::W`](W) writer structure"]
impl crate::Writable for RcsrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RCSR to value 0"]
impl crate::Resettable for RcsrSpec {}
